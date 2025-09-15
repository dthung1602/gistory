import { useContext, useState } from "react";

import GenerateIcon from "../asset/auto-generate-svgrepo-com.svg?react";
import { FormContext, ToastContext } from "../context.tsx";

function GenerateButton() {
  const { addToast } = useContext(ToastContext);
  const { inputErrors, name, username, branch, email, startDate, timezone, data } = useContext(FormContext);
  const [loading, setLoading] = useState(false);

  const disabled = Object.values(inputErrors).some(Boolean);

  const handleClick = () => {
    if (disabled || loading) return;

    const emptyField = data.length == 0 || [name, username, branch, email, startDate, timezone].includes(null);

    if (emptyField) {
      addToast({ type: "success", content: "Please fill in all fields" });
      return;
    }

    // TODO call api to
    console.log("generate");

    setLoading(true);
    setTimeout(() => {
      setLoading(false);
      console.log("callback");

      addToast({ type: "info", content: "Done" });
    }, 10000);
  };

  console.log(inputErrors, name, username, branch, email, startDate, timezone, data);

  let buttonClass = "btn";
  if (disabled) {
    buttonClass += " btn-disabled";
  }
  if (loading) {
    buttonClass += " gradient-animated text-secondary-content";
  } else {
    buttonClass += " btn-secondary";
  }

  return (
    <div className="flex justify-center mt-8">
      <button onClick={handleClick} className={buttonClass}>
        {loading ? <span className="loading loading-spinner loading-sm" /> : <GenerateIcon className="w-6 h-6" />}
        Generate Repo
      </button>
    </div>
  );
}

export default GenerateButton;
