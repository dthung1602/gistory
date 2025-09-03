import PatternInput from "./PatternInput.tsx";

function Manual() {
  return (
    <PatternInput
      title="Manual Input"
      subtitle="Mannually set the commits for each day within a date range"
    >
      <fieldset className="fieldset ">
        <legend className="fieldset-legend">Start date</legend>
        <input type="date" className="input w-full" />
      </fieldset>
    </PatternInput>
  );
}

export default Manual;
