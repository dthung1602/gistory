import { useContext, useEffect, useState } from "react";

import api from "../api.tsx";
import { VisualizerMethod } from "../constants.ts";
import { ToastContext } from "../context.tsx";
import { useFileInput, usePreviewData, useStartDateInput } from "../hooks.ts";
import type { FileUploadResult, PreviewResult, SelectPatternTabProp } from "../types.ts";
import InputDate from "./InputDate.tsx";
import InputFile from "./InputFile.tsx";
import PatternTab from "./PatternTab.tsx";

function TextFilePattern({ updatePreviewData }: SelectPatternTabProp) {
  const { addToast } = useContext(ToastContext);

  const [startDate, onStartDateChange, startDateErr] = useStartDateInput({ mustBeSunday: true });
  const { file, onChange, fileId, setFileId, fileErr, setFileErr } = useFileInput();

  const [data, setData, setDataAtIndex] = usePreviewData(updatePreviewData);

  const [uploading, setUploading] = useState(false);
  const [loading, setLoading] = useState(false);

  const hasError = startDateErr || fileErr;

  useEffect(() => {
    if (file == null) return;

    setUploading(true);
    const [reqPromise, controller] = api.upload(file);

    reqPromise
      .then(async res => {
        const { uuid } = (await res.json()) as FileUploadResult;
        setFileId(uuid);
      })
      .catch(api.errHandler(addToast, setFileErr))
      .finally(() => setUploading(false));

    return () => {
      controller.abort("component dismount");
      setUploading(false);
    };
  }, [addToast, file, setFileErr, setFileId]);

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
  }, [fileId, hasError, setData, setFileErr, startDate]);

  return (
    <PatternTab
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
      loading={loading}
    >
      <InputDate legend="Start date" date={startDate} onChange={onStartDateChange} error={startDateErr} />
      <InputFile legend="Pattern file" accept="text/plain" onChange={onChange} error={fileErr} uploading={uploading} />
    </PatternTab>
  );
}

export default TextFilePattern;
