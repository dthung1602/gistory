import { useCallback, useEffect, useState } from "react";

import api from "../api.tsx";
import { VisualizerMethod } from "../constants.ts";
import { useDateInput, useFileInput, usePreviewData } from "../hooks.ts";
import type { FileUploadResult, PreviewResult } from "../types.ts";
import InputDate from "./InputDate.tsx";
import InputFile from "./InputFile.tsx";
import PatternInput from "./PatternInput.tsx";

function TextFilePattern() {
  const [startDate, onStartDateChange, startDateErr] = useDateInput({ mustBeSunday: true });
  const { file, onChange, fileId, setFileId, fileErr, setFileErr } = useFileInput();

  const [data, setData, setDataAtIndex] = usePreviewData();

  const [loading, setLoading] = useState(false);
  const handleGenerate = useCallback(() => {
    // TODO
    console.log("click");
  }, []);

  const hasError = startDateErr || fileErr;

  useEffect(() => {
    if (file == null) return;

    setLoading(true);
    const [reqPromise, controller] = api.upload(file);

    reqPromise
      .then(async res => {
        const { uuid } = (await res.json()) as FileUploadResult;
        setFileId(uuid);
      })
      .catch((e: Error) => setFileErr("Error: " + e))
      .finally(() => setLoading(false));

    return () => {
      controller.abort("component dismount");
      setLoading(false);
    };
  }, [file]);

  useEffect(() => {
    if (!fileId || hasError) return;

    setLoading(true);
    const [reqPromise, controller] = api.preview({
      method: VisualizerMethod.PatternFile,
      start_date: startDate,
      input_file: fileId,
    });

    reqPromise
      .then(async res => {
        const { data } = (await res.json()) as PreviewResult;
        setData(data);
      })
      .catch((e: Error) => setFileErr("Error: " + e))
      .finally(() => setLoading(false));

    return () => {
      controller.abort("component dismount");
      setLoading(false);
    };
  }, [fileId, startDate]);

  return (
    <PatternInput
      title="Text File Pattern"
      subtitle={
        <span>
          Upload a text file containing a pattern to draw on your commit graph. <br />
          The text file must have fewer than 7 rows. Use 0-4 to indicate commit counts
        </span>
      }
      startDate={startDate}
      data={data}
      setDataAtIndex={setDataAtIndex}
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputFile legend="Pattern file" accept="text/plain" onChange={onChange} error={fileErr} />
    </PatternInput>
  );
}

export default TextFilePattern;
