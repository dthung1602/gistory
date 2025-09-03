function InputCommitCount() {
  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">Commit count</legend>
      <select className="select w-full">
        <option value="Zero">Zero</option>
        <option value="Few">Few</option>
        <option value="Some">Some</option>
        <option value="Many">Many</option>
        <option value="ALot">A Lot</option>
      </select>
    </fieldset>
  );
}

export default InputCommitCount;
