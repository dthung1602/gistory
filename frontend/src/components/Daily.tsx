import InputCommitCount from "./InputCommitCount.tsx";
import PatternInput from "./PatternInput.tsx";

function Daily() {
  return (
    <PatternInput
      title="Daily Commits Pattern"
      subtitle="Generate a pattern with a fixed number of commits every day within a date range"
    >
      <fieldset className="fieldset ">
        <legend className="fieldset-legend">Start date</legend>
        <input type="date" className="input w-full" />
      </fieldset>

      <fieldset className="fieldset">
        <legend className="fieldset-legend">End date</legend>
        <input type="date" className="input w-full" />
      </fieldset>

      <InputCommitCount />
    </PatternInput>
  );
}

export default Daily;
