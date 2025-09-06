import { Font } from "../constants.ts";
import type { SelectInputProp } from "../types.ts";

function InputFont({ value, onChange }: SelectInputProp<Font>) {
  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">Font</legend>
      <select className="select w-full" value={value} onChange={onChange}>
        <option value={Font.SubwayTracker}>Subway Tracker</option>
      </select>
    </fieldset>
  );
}

export default InputFont;
