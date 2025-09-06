import { useDateInput } from "../hooks.ts";
import InputDate from "./InputDate.tsx";
import PatternInput from "./PatternInput.tsx";

function Manual() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput();

  return (
    <PatternInput title="Manual Input" subtitle="Mannually set the commits for each day within a date range">
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
    </PatternInput>
  );
}

export default Manual;
