import PatternInput from "./PatternInput.tsx";

function Random() {
  return (
    <PatternInput
      title="Random Commits Pattern"
      subtitle="Generate a pattern with a random number of commits each day within a date range"
    >
      <fieldset className="fieldset ">
        <legend className="fieldset-legend">Start date</legend>
        <input type="date" className="input w-full" />
      </fieldset>

      <fieldset className="fieldset">
        <legend className="fieldset-legend">End date</legend>
        <input type="date" className="input w-full" />
      </fieldset>
    </PatternInput>
  );
}

export default Random;
