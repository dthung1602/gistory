import type { RefObject } from "react";

import type { OnInputChange } from "../types.ts";

type InputTextProp = {
  legend: string;
  placeholder: string;
  onChange: OnInputChange;
  inputRef: RefObject<HTMLInputElement | null>;
};

function InputText({ legend, placeholder, onChange, inputRef }: InputTextProp) {
  return (
    <fieldset className="fieldset col-span-2">
      <legend className="fieldset-legend">{legend}</legend>
      <input className="input w-full" placeholder={placeholder} onChange={onChange} ref={inputRef} />
    </fieldset>
  );
}

export default InputText;
