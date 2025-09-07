import { useCallback, useEffect, useState } from "react";

import api from "../api.tsx";
import { VisualizerMethod } from "../constants.ts";
import { useCommitCountInput, useDateInput, useFontInput, usePreviewData, useTextInput } from "../hooks.ts";
import type { PreviewResult } from "../types.ts";
import InputCommitCount from "./InputCommitCount.tsx";
import InputDate from "./InputDate.tsx";
import InputFont from "./InputFont.tsx";
import InputText from "./InputText.tsx";
import PatternInput from "./PatternInput.tsx";

function Text() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput();
  const [commitCount, onCommitCountChange] = useCommitCountInput();
  const [font, onFontChange] = useFontInput();
  const [text, onTextChange, textInputRef] = useTextInput(250);

  const [data, setData, setDataAtIndex] = usePreviewData();

  const hasError = !!startDateErr || !text;

  const [loading, setLoading] = useState(false);
  const handleGenerate = useCallback(() => {
    // TODO
    console.log("click");
  }, []);

  useEffect(() => {
    if (hasError) return;

    setLoading(true);

    const [resPromise, controller] = api.preview({
      method: VisualizerMethod.Text,
      start_date: startDate,
      commit_count: commitCount,
      font,
      text,
    });

    resPromise
      .then(async res => {
        if (controller.signal.aborted) return;
        const { data } = (await res.json()) as PreviewResult;
        setData(data);
      })
      .catch(() => {})
      .finally(() => setLoading(false));

    return () => {
      controller.abort("component dismount");
      setLoading(false);
    };
  }, [startDate, commitCount, font, text, setData]);

  return (
    <PatternInput
      title="Text Pattern"
      subtitle="Display text on your commit graph. The text will be converted into a commit pattern"
      startDate={startDate}
      data={data}
      setDataAtIndex={setDataAtIndex}
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputCommitCount value={commitCount} onChange={onCommitCountChange} />
      <InputFont value={font} onChange={onFontChange} />
      <InputText
        legend="Text"
        placeholder="Keep it under 64 chars"
        maxLength={64}
        onChange={onTextChange}
        inputRef={textInputRef}
      />
    </PatternInput>
  );
}

export default Text;
