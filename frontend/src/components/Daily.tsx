import { useCommitCountInput, useDateInput } from "../hooks.ts";
import InputCommitCount from "./InputCommitCount.tsx";
import InputDate from "./InputDate.tsx";
import PatternInput from "./PatternInput.tsx";

function Daily() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput();
  const [endDate, onEndDateChange, endDateErr] = useDateInput({ minDate: startDate });
  const [commitCount, onCommitCountChange] = useCommitCountInput();

  return (
    <PatternInput
      title="Daily Commits Pattern"
      subtitle="Generate a pattern with a fixed number of commits every day within a date range"
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputDate legend="End date" date={endDate} onDateChange={onEndDateChange} dateErr={endDateErr} />
      <InputCommitCount value={commitCount} onChange={onCommitCountChange} />
    </PatternInput>
  );
}

export default Daily;
