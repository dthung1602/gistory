import type { OnInputChange } from "../types.ts";

type InputFileProp = {
  accept?: string;
  legend: string;
  error?: string;
  onChange: OnInputChange;
};

function InputFile({ accept, legend, error, onChange }: InputFileProp) {
  return (
    <fieldset className="fieldset">
      <legend className={`fieldset-legend ${error ? "text-error" : ""}`}>{legend}</legend>
      <input type="file" className="file-input w-full" accept={accept} onChange={onChange} />
      <p className="text-error">{error}</p>
    </fieldset>
  );
}

export default InputFile;
