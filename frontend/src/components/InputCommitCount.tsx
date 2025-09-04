import { CommitCount } from "../constants.ts";

function InputCommitCount() {
  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">Commit count</legend>
      <select className="select w-full">
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
