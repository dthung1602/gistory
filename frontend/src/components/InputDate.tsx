import type { OnInputChange } from "../types.ts";

type InputStartDateProp = {
  legend?: string;
  date: string;
  onChange: OnInputChange;
  error: string | null;
};

function InputDate({ date, onChange, error, legend = "Date" }: InputStartDateProp) {
  return (
    <fieldset className="fieldset ">
      <legend className={`fieldset-legend ${error ? "text-error" : ""}`}>{legend}</legend>
      <input type="date" className={`input w-full ${error ? "input-error" : ""}`} value={date} onChange={onChange} />
      <p className="text-error">{error}</p>
    </fieldset>
  );
}

export default InputDate;
