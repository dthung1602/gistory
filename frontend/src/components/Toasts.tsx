import type { ToastData } from "../context.ts";

function Toasts() {
  const toasts: ToastData[] = [];
  return (
    <div className="toast toast-top toast-center">
      {toasts.map(({ type, content, key }) => (
        <div key={key} className={`alert alert-${type}`}>
          {content}
        </div>
      ))}
    </div>
  );
}
export default Toasts;
