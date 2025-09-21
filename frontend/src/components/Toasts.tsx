import { useContext } from "react";

import { ToastContext } from "../context.tsx";

function Toasts() {
  const { toasts } = useContext(ToastContext);
  return (
    <div className="toast toast-top toast-end">
      {/* the build process somehow removes the alert classes, so we add them here*/}
      <div className="hidden alert-success alert-info alert-warning alert-error" />

      {toasts.map(({ type, content, key }) => (
        <div key={key} className={`alert alert-${type}`}>
          {content}
        </div>
      ))}
    </div>
  );
}
export default Toasts;
