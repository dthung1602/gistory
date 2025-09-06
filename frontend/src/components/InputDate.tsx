import type { OnInputChange } from "../types.ts";

type InputStartDateProp = {
  legend?: string;
  date: string;
  onDateChange: OnInputChange;
  dateErr: string;
};

function InputDate({ date, onDateChange, dateErr, legend = "Date" }: InputStartDateProp) {
  return (
    <fieldset className="fieldset ">
      <legend className={`fieldset-legend ${dateErr ? "text-error" : ""}`}>{legend}</legend>
      <input
        type="date"
        className={`input w-full ${dateErr ? "input-error" : ""}`}
        value={date}
        onChange={onDateChange}
      />
      <p className="text-error">{dateErr}</p>
    </fieldset>
  );
}

export default InputDate;
