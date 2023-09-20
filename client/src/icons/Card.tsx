import IconWrapper from './Wrapper';

const RawIcon = (
  <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path
      d="M3.75 3.125C3.08696 3.125 2.45107 3.38839 1.98223 3.85723C1.51339 4.32607 1.25 4.96196 1.25 5.625V6.25H18.75V5.625C18.75 4.96196 18.4866 4.32607 18.0178 3.85723C17.5489 3.38839 16.913 3.125 16.25 3.125H3.75Z"
      fill="currentColor"
    />
    <path
      fillRule="evenodd"
      clipRule="evenodd"
      d="M18.75 8.125H1.25V14.375C1.25 15.038 1.51339 15.6739 1.98223 16.1428C2.45107 16.6116 3.08696 16.875 3.75 16.875H16.25C16.913 16.875 17.5489 16.6116 18.0178 16.1428C18.4866 15.6739 18.75 15.038 18.75 14.375V8.125ZM3.75 11.25C3.75 11.0842 3.81585 10.9253 3.93306 10.8081C4.05027 10.6908 4.20924 10.625 4.375 10.625H9.375C9.54076 10.625 9.69973 10.6908 9.81694 10.8081C9.93415 10.9253 10 11.0842 10 11.25C10 11.4158 9.93415 11.5747 9.81694 11.6919C9.69973 11.8092 9.54076 11.875 9.375 11.875H4.375C4.20924 11.875 4.05027 11.8092 3.93306 11.6919C3.81585 11.5747 3.75 11.4158 3.75 11.25ZM4.375 13.125C4.20924 13.125 4.05027 13.1908 3.93306 13.3081C3.81585 13.4253 3.75 13.5842 3.75 13.75C3.75 13.9158 3.81585 14.0747 3.93306 14.1919C4.05027 14.3092 4.20924 14.375 4.375 14.375H6.875C7.04076 14.375 7.19973 14.3092 7.31694 14.1919C7.43415 14.0747 7.5 13.9158 7.5 13.75C7.5 13.5842 7.43415 13.4253 7.31694 13.3081C7.19973 13.1908 7.04076 13.125 6.875 13.125H4.375Z"
      fill="currentColor"
    />
  </svg>
);

const BoxedIcon = (
  <svg
    width="20"
    height="20"
    viewBox="0 0 20 20"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M3.75 3.125C3.08696 3.125 2.45107 3.38839 1.98223 3.85723C1.51339 4.32607 1.25 4.96196 1.25 5.625V6.25H18.75V5.625C18.75 4.96196 18.4866 4.32607 18.0178 3.85723C17.5489 3.38839 16.913 3.125 16.25 3.125H3.75Z"
      fill="currentColor"
    />
    <path
      fillRule="evenodd"
      clipRule="evenodd"
      d="M18.75 8.125H1.25V14.375C1.25 15.038 1.51339 15.6739 1.98223 16.1428C2.45107 16.6116 3.08696 16.875 3.75 16.875H16.25C16.913 16.875 17.5489 16.6116 18.0178 16.1428C18.4866 15.6739 18.75 15.038 18.75 14.375V8.125ZM3.75 11.25C3.75 11.0842 3.81585 10.9253 3.93306 10.8081C4.05027 10.6908 4.20924 10.625 4.375 10.625H9.375C9.54076 10.625 9.69973 10.6908 9.81694 10.8081C9.93415 10.9253 10 11.0842 10 11.25C10 11.4158 9.93415 11.5747 9.81694 11.6919C9.69973 11.8092 9.54076 11.875 9.375 11.875H4.375C4.20924 11.875 4.05027 11.8092 3.93306 11.6919C3.81585 11.5747 3.75 11.4158 3.75 11.25ZM4.375 13.125C4.20924 13.125 4.05027 13.1908 3.93306 13.3081C3.81585 13.4253 3.75 13.5842 3.75 13.75C3.75 13.9158 3.81585 14.0747 3.93306 14.1919C4.05027 14.3092 4.20924 14.375 4.375 14.375H6.875C7.04076 14.375 7.19973 14.3092 7.31694 14.1919C7.43415 14.0747 7.5 13.9158 7.5 13.75C7.5 13.5842 7.43415 13.4253 7.31694 13.3081C7.19973 13.1908 7.04076 13.125 6.875 13.125H4.375Z"
      fill="currentColor"
    />
  </svg>
);

export default IconWrapper(RawIcon, BoxedIcon);
