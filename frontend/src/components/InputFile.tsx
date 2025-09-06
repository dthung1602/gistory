import type { OnInputChange } from "../types.ts";

type InputFileProp = {
  accept?: string;
  legend: string;
  onChange: OnInputChange;
};

function InputFile({ accept, legend, onChange }: InputFileProp) {
  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">{legend}</legend>
      <input type="file" className="file-input w-full" accept={accept} onChange={onChange} />
    </fieldset>
  );
}

export default InputFile;
