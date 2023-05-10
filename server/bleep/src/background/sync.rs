use tokio::sync::OwnedSemaphorePermit;
use tracing::{debug, error, info};

use crate::{
    indexes,
    remotes::RemoteError,
    repo::{Backend, RepoError, RepoMetadata, RepoRef, Repository, SyncStatus},
    Application,
};

use std::{path::PathBuf, sync::Arc};

use super::control::SyncPipes;

pub(crate) struct SyncHandle {
    pub(crate) reporef: RepoRef,
    pub(crate) app: Application,
    pipes: SyncPipes,
    status: tokio::sync::broadcast::Sender<super::Progress>,
    exited: flume::Sender<SyncStatus>,
}

type Result<T> = std::result::Result<T, SyncError>;
#[derive(thiserror::Error, Debug)]
pub(super) enum SyncError {
    #[error("no keys for backend: {0:?}")]
    NoKeysForBackend(Backend),

    #[error("path not allowed: {0:?}")]
    PathNotAllowed(PathBuf),

    #[error("indexing failed: {0:?}")]
    Indexing(RepoError),

    #[error("sync failed: {0:?}")]
    Sync(RemoteError),

    #[error("file cache cleanup failed: {0:?}")]
    State(RepoError),

    #[error("file cache cleanup failed: {0:?}")]
    FileCache(RepoError),

    #[error("folder cleanup failed: path: {0:?}, error: {1}")]
    RemoveLocal(PathBuf, std::io::Error),

    #[error("tantivy: {0:?}")]
    Tantivy(anyhow::Error),
}

impl PartialEq for SyncHandle {
    fn eq(&self, other: &Self) -> bool {
        self.reporef == other.reporef
    }
}

impl Drop for SyncHandle {
    fn drop(&mut self) {
        let status = self
            .app
            .repo_pool
            .update(&self.reporef, |_k, v| {
                use SyncStatus::*;
                v.sync_status = match &v.sync_status {
                    Indexing | Syncing => Error {
                        message: "unknown".into(),
                    },
                    other => other.clone(),
                };

                v.sync_status.clone()
            })
            .expect("the repo has been deleted from the db?");

        info!(?status, %self.reporef, "normalized status after sync");
        self.exited.send(status).expect("pipe closed prematurely");
    }
}

impl SyncHandle {
    pub(super) fn new(
        app: Application,
        reporef: RepoRef,
        status: tokio::sync::broadcast::Sender<super::Progress>,
    ) -> (Arc<Self>, flume::Receiver<SyncStatus>) {
        let (exited, exit_signal) = flume::bounded(1);
        let pipes = SyncPipes::default();

        (
            Self {
                app,
                pipes,
                reporef,
                status,
                exited,
            }
            .into(),
            exit_signal,
        )
    }

    /// The permit that's taken here is exclusively for parallelism control.
    pub(super) async fn run(&self, _permit: OwnedSemaphorePermit) -> Result<SyncStatus> {
        debug!(?self.reporef, "syncing repo");
        let Application { ref repo_pool, .. } = self.app;

        // skip git operations if the repo has been marked as removed
        // if the ref is non-existent, sync it and add it to the pool
        let removed = repo_pool
            .read_async(&self.reporef, |_k, v| v.sync_status == SyncStatus::Removed)
            .await
            .unwrap_or(false);

        if !removed {
            if let Err(err) = self.sync().await {
                error!(?err, ?self.reporef, "failed to sync repository");
                return Err(err);
            }
        }

        let indexed = self.index().await;
        let status = repo_pool
            .update_async(&self.reporef, |_k, repo| match indexed {
                Ok(Some(state)) => {
                    info!("commit complete; indexing done");
                    repo.sync_done_with(state);
                    SyncStatus::Done
                }
                Ok(None) => SyncStatus::Done,
                Err(err) => {
                    repo.sync_status = SyncStatus::Error {
                        message: err.to_string(),
                    };
                    error!(?err, ?self.reporef, "failed to index repository");
                    repo.sync_status.clone()
                }
            })
            .await
            .unwrap();

        Ok(status)
    }

    async fn index(&self) -> Result<Option<Arc<RepoMetadata>>> {
        use SyncStatus::*;
        let Application {
            ref config,
            ref indexes,
            ref repo_pool,
            ..
        } = self.app;

        let writers = indexes.writers().await.map_err(SyncError::Tantivy)?;
        let repo = repo_pool
            .read_async(&self.reporef, |_k, v| v.clone())
            .await
            .unwrap();

        let indexed = match repo.sync_status {
            Uninitialized | Syncing | Indexing => return Ok(None),
            Removed => {
                repo_pool.remove(&self.reporef);
                let deleted = self.delete_repo_indexes(&repo, &writers).await;
                if deleted.is_ok() {
                    writers.commit().await.map_err(SyncError::Tantivy)?;
                    config
                        .source
                        .save_pool(repo_pool.clone())
                        .map_err(SyncError::State)?;
                }

                return deleted.map(|_| None);
            }
            RemoteRemoved => {
                // Note we don't clean up here, leave the
                // barebones behind.
                //
                // This is to be able to report to the user that
                // something happened, and let them clean up in a
                // subsequent action.
                return Ok(None);
            }
            _ => {
                repo_pool
                    .update_async(&self.reporef, |_, v| v.sync_status = Indexing)
                    .await
                    .unwrap();

                {
                    let reporef = self.reporef.clone();
                    let status = self.status.clone();
                    writers
                        .index(
                            &self.reporef,
                            &repo,
                            Arc::new(move |p: u8| {
                                status.send((reporef.clone(), 1, p));
                            }) as Arc<dyn Fn(u8) + Send + Sync>,
                        )
                        .await
                }
            }
        };

        if indexed.is_ok() {
            writers.commit().await.map_err(SyncError::Tantivy)?;
            config
                .source
                .save_pool(repo_pool.clone())
                .map_err(SyncError::State)?;
        }

        indexed.map_err(SyncError::Indexing).map(Some)
    }

    async fn sync(&self) -> Result<()> {
        let repo = self.reporef.clone();
        let backend = repo.backend();
        let creds = match self.app.credentials.for_repo(&repo) {
            Some(creds) => creds,
            None => {
                let Some(path) = repo.local_path() else {
		    return Err(SyncError::NoKeysForBackend(backend));
		};

                if !self.app.allow_path(&path) {
                    return Err(SyncError::PathNotAllowed(path));
                }

                self.app
                    .repo_pool
                    .entry_async(repo.to_owned())
                    .await
                    .or_insert_with(|| Repository::local_from(&repo));

                // we _never_ touch the git repositories of local repos
                return Ok(());
            }
        };

        let synced = creds.sync(self).await;
        if let Err(RemoteError::RemoteNotFound) = synced {
            self.app
                .repo_pool
                .update_async(&repo, |_, v| v.sync_status = SyncStatus::RemoteRemoved)
                .await
                .unwrap();

            error!(?repo, "remote repository removed; disabling local syncing");

            // we want indexing to pick this up later and handle the new state
            // all local cleanups are done, so everything should be consistent
            return Ok(());
        }

        synced.map_err(SyncError::Sync)
    }

    async fn delete_repo_indexes(
        &self,
        repo: &Repository,
        writers: &indexes::GlobalWriteHandleRef<'_>,
    ) -> Result<()> {
        let Application {
            ref config,
            ref semantic,
            ..
        } = self.app;

        if let Some(semantic) = semantic {
            semantic
                .delete_points_by_path(&self.reporef.to_string(), std::iter::empty())
                .await;
        }

        repo.delete_file_cache(&config.index_dir)
            .map_err(SyncError::FileCache)?;

        if !self.reporef.is_local() {
            tokio::fs::remove_dir_all(&repo.disk_path)
                .await
                .map_err(|e| SyncError::RemoveLocal(repo.disk_path.clone(), e))?;
        }

        for handle in writers {
            handle.delete(repo);
        }

        Ok(())
    }
}