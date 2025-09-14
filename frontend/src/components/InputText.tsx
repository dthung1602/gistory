import type { RefObject } from "react";

import type { OnInputChange } from "../types.ts";

type InputTextProp = {
  legend: string;
  placeholder?: string;
  maxLength?: number;
  onChange: OnInputChange;
  inputRef: RefObject<HTMLInputElement | null>;
  type?: string;
  error?: string | null;
  className?: string;
};

function InputText({
  legend,
  placeholder,
  maxLength,
  onChange,
  inputRef,
  type = "text",
  error = "",
  className = "",
}: InputTextProp) {
  return (
    <fieldset className={"fieldset " + className}>
      <legend className={`fieldset-legend ${error ? "text-error" : ""}`}>{legend}</legend>
      <input
        maxLength={maxLength}
        className={`input w-full ${error ? "input-error" : ""}`}
        placeholder={placeholder}
        onChange={onChange}
        ref={inputRef}
        type={type}
      />
      <p className="text-error">{error}</p>
    </fieldset>
  );
}

export default InputText;
