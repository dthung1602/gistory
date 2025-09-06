import { useCallback, useEffect, useState } from "react";

import { NEXT_6_MONTH_STR, TODAY_STR } from "../constants.ts";
import { useCommitCountInput, useDateInput, usePreviewData } from "../hooks.ts";
import { dayDiff } from "../utils.ts";
import InputCommitCount from "./InputCommitCount.tsx";
import InputDate from "./InputDate.tsx";
import PatternInput from "./PatternInput.tsx";

function Daily() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput({ defaultValue: TODAY_STR });
  const [endDate, onEndDateChange, endDateErr] = useDateInput({ defaultValue: NEXT_6_MONTH_STR, minDate: startDate });
  const [commitCount, onCommitCountChange] = useCommitCountInput();
  const [data, setData, setDataAtIndex] = usePreviewData();

  const hasError = startDateErr || endDateErr;

  const [loading, setLoading] = useState(false);
  const handleGenerate = useCallback(() => {
    // TODO
    console.log("click");
  }, []);

  useEffect(() => {
    if (hasError) return;
    const newData = Array(dayDiff(endDate, startDate) + 1).fill(commitCount);
    setData(newData);
  }, [startDate, endDate, commitCount]);

  return (
    <PatternInput
      title="Daily Commits Pattern"
      subtitle="Generate a pattern with a fixed number of commits every day within a date range"
      startDate={startDate}
      data={data}
      setDataAtIndex={setDataAtIndex}
      isGenerating={loading}
      onGenerate={hasError ? undefined : handleGenerate}
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputDate legend="End date" date={endDate} onDateChange={onEndDateChange} dateErr={endDateErr} />
      <InputCommitCount value={commitCount} onChange={onCommitCountChange} />
    </PatternInput>
  );
}

export default Daily;
