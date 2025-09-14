import { useEffect } from "react";

import { NEXT_6_MONTH_STR, TODAY_STR } from "../constants.ts";
import { useCommitCountInput, useDateInput, usePreviewData, useStartDateInput } from "../hooks.ts";
import type { SelectPatternTabProp } from "../types.ts";
import { dayDiff } from "../utils.ts";
import InputCommitCount from "./InputCommitCount.tsx";
import InputDate from "./InputDate.tsx";
import PatternTab from "./PatternTab.tsx";

function Daily({ updatePreviewData }: SelectPatternTabProp) {
  const [startDate, onStartDateChange, startDateErr] = useStartDateInput({ defaultValue: TODAY_STR });
  const [endDate, onEndDateChange, endDateErr] = useDateInput({ defaultValue: NEXT_6_MONTH_STR, minDate: startDate });
  const [commitCount, onCommitCountChange] = useCommitCountInput();
  const [data, setData, setDataAtIndex] = usePreviewData(updatePreviewData);

  const hasError = startDateErr || endDateErr;

  useEffect(() => {
    if (hasError) return;
    const newData = Array(dayDiff(endDate, startDate) + 1).fill(commitCount);
    setData(newData);
  }, [startDate, endDate, commitCount, hasError, setData]);

  return (
    <PatternTab
      title="Daily Commits Pattern"
      subtitle="Generate a pattern with a fixed number of commits every day within a date range"
      startDate={startDate}
      data={data}
      setDataAtIndex={setDataAtIndex}
    >
      <InputDate legend="Start date" date={startDate} onChange={onStartDateChange} error={startDateErr} />
      <InputDate legend="End date" date={endDate} onChange={onEndDateChange} error={endDateErr} />
      <InputCommitCount value={commitCount} onChange={onCommitCountChange} />
    </PatternTab>
  );
}

export default Daily;
