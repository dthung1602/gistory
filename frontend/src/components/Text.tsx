import PatternInput from "./PatternInput.tsx";
import InputCommitCount from "./InputCommitCount.tsx";

function Text() {
  return (
    <PatternInput
      title="Text Pattern"
      subtitle="Display text on your commit graph. The text will be converted into a commit pattern"
    >
      <fieldset className="fieldset ">
        <legend className="fieldset-legend">Start date</legend>
        <input type="date" className="input w-full" />
      </fieldset>

      <InputCommitCount />

      <fieldset className="fieldset">
        <legend className="fieldset-legend">Font</legend>
        <select className="select w-full">
          <option value="Zero">SubwayTracker</option>
        </select>
      </fieldset>

      <fieldset className="fieldset col-span-2">
        <legend className="fieldset-legend">Text</legend>
        <input className="input w-full" placeholder="Keep it short" />
      </fieldset>
    </PatternInput>
  );
}

export default Text;
