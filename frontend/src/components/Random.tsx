import { useDateInput } from "../hooks.ts";
import InputDate from "./InputDate.tsx";
import PatternInput from "./PatternInput.tsx";

function Random() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput();
  const [endDate, onEndDateChange, endDateErr] = useDateInput({ minDate: startDate });

  return (
    <PatternInput
      title="Random Commits Pattern"
      subtitle="Generate a pattern with a random number of commits each day within a date range"
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputDate legend="End date" date={endDate} onDateChange={onEndDateChange} dateErr={endDateErr} />
    </PatternInput>
  );
}

export default Random;
