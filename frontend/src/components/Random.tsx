import { useCallback, useEffect, useState } from "react";

import { ALL_COMMIT_COUNT, NEXT_6_MONTH_STR, TODAY_STR } from "../constants.ts";
import { useDateInput, usePreviewData } from "../hooks.ts";
import { dayDiff } from "../utils.ts";
import InputDate from "./InputDate.tsx";
import PatternInput from "./PatternInput.tsx";

function Random() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput({ defaultValue: TODAY_STR });
  const [endDate, onEndDateChange, endDateErr] = useDateInput({ defaultValue: NEXT_6_MONTH_STR, minDate: startDate });
  const [data, setData, setDataAtIndex] = usePreviewData();

  const hasError = startDateErr || endDateErr;

  const [loading, setLoading] = useState(false);
  const handleGenerate = useCallback(() => {
    // TODO
    console.log("click");
  }, []);

  useEffect(() => {
    if (hasError) return;
    const total = dayDiff(endDate, startDate) + 1;
    const newData = [];
    for (let i = 0; i < total; i++) {
      newData.push(randomCommitCount());
    }
    setData(newData);
  }, [startDate, endDate]);

  return (
    <PatternInput
      title="Random Commits Pattern"
      subtitle="Generate a pattern with a random number of commits each day within a date range"
      startDate={startDate}
      data={data}
      setDataAtIndex={setDataAtIndex}
      isGenerating={loading}
      onGenerate={hasError ? undefined : handleGenerate}
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputDate legend="End date" date={endDate} onDateChange={onEndDateChange} dateErr={endDateErr} />
    </PatternInput>
  );
}

function randomCommitCount() {
  const idx = Math.floor(Math.random() * ALL_COMMIT_COUNT.length);
  return ALL_COMMIT_COUNT[idx];
}

export default Random;
