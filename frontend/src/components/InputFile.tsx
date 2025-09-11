import * as React from "react";

import type { OnInputChange } from "../types.ts";

type InputFileProp = {
  accept?: string;
  legend: string;
  error?: string;
  onChange: OnInputChange;
  uploading?: boolean;
};

function InputFile({ accept, legend, error, onChange, uploading = false }: InputFileProp) {
  return (
    <fieldset className="fieldset">
      <legend className={`fieldset-legend ${error ? "text-error" : ""}`}>{legend}</legend>
      <input type="file" className="file-input w-full" accept={accept} onChange={onChange} />
      {uploading ? (
        <span className="loading loading-dots loading-sm text-secondary" />
      ) : (
        <p className="text-error">{error}</p>
      )}
    </fieldset>
  );
}

export default InputFile;
