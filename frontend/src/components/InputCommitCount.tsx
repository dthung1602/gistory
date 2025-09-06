import { CommitCount } from "../constants.ts";
import type { SelectInputProp } from "../types.ts";

function InputCommitCount({ value, onChange }: SelectInputProp<CommitCount>) {
  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">Commit count</legend>
      <select className="select w-full" value={value} onChange={onChange}>
        <option value={CommitCount.Zero}>Zero</option>
        <option value={CommitCount.Few}>Few</option>
        <option value={CommitCount.Some}>Some</option>
        <option value={CommitCount.Many}>Many</option>
        <option value={CommitCount.ALot}>A Lot</option>
      </select>
    </fieldset>
  );
}

export default InputCommitCount;
