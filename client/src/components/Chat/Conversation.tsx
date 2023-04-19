import React, { useEffect, useRef } from 'react';
import {
  ChatMessage,
  ChatMessageAuthor,
  ChatMessageType,
} from '../../types/general';
import { Checkmark, MagnifyTool } from '../../icons';
import Message from './ConversationMessage';

type Props = {
  conversation: ChatMessage[];
};

const Conversation = ({ conversation }: Props) => {
  const messagesRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    messagesRef.current?.scrollTo({
      left: 0,
      top: messagesRef.current?.scrollHeight,
      behavior: 'smooth',
    });
  }, [conversation]);

  return (
    <div
      className="w-full flex flex-col gap-3 pb-3 overflow-auto max-h-80"
      ref={messagesRef}
    >
      {conversation.map((m, i) => (
        <>
          {m.author === ChatMessageAuthor.Server &&
            m.type === ChatMessageType.Answer && (
              <div className="group-custom">
                <div className="hidden group-custom-hover:flex flex-col gap-2 text-gray-400 caption">
                  {m.loadingSteps.map((s, i) => (
                    <div className="flex gap-2 px-4 items-center" key={i}>
                      <div className="h-5">
                        <MagnifyTool />
                      </div>
                      <p>{s}</p>
                    </div>
                  ))}
                </div>
                <div className="flex gap-2 px-4 items-center group-custom-hover:hidden">
                  {!m.isLoading ? (
                    <div className="text-success-500 h-5">
                      <Checkmark />
                    </div>
                  ) : (
                    <div className="text-gray-400 h-5">
                      <MagnifyTool />
                    </div>
                  )}
                  <p className="caption text-gray-400 flex-1">
                    {m.isLoading
                      ? m.loadingSteps[m.loadingSteps.length - 1]
                      : 'Answer Ready'}
                  </p>
                </div>
              </div>
            )}
          {m.text || (m.author === ChatMessageAuthor.Server && m.error) ? (
            <Message
              isHistory={false}
              key={i}
              author={m.author}
              message={
                m.text ||
                (m.author === ChatMessageAuthor.Server && m.error) ||
                ''
              }
            />
          ) : null}
        </>
      ))}
    </div>
  );
};

export default Conversation;
