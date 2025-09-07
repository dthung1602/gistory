import { useCallback, useContext, useEffect, useState } from "react";

import api from "../api.tsx";
import { VisualizerMethod } from "../constants.ts";
import { ToastContext } from "../context.ts";
import { useDateInput, useFileInput, usePreviewData } from "../hooks.ts";
import type { FileUploadResult, PreviewResult } from "../types.ts";
import InputDate from "./InputDate.tsx";
import InputFile from "./InputFile.tsx";
import PatternTab from "./PatternTab.tsx";

function Image() {
  const { addToast } = useContext(ToastContext);

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
      .catch(api.errHandler(addToast, setFileErr))
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
      method: VisualizerMethod.Image,
      start_date: startDate,
      input_file: fileId,
    });

    reqPromise
      .then(async res => {
        const { data } = (await res.json()) as PreviewResult;
        setData(data);
      })
      .catch(api.errHandler(addToast, setFileErr))
      .finally(() => setLoading(false));

    return () => {
      controller.abort("component dismount");
      setLoading(false);
    };
  }, [fileId, startDate]);

  return (
    <PatternTab
      title="Image Pattern"
      subtitle="Upload an image to convert into a commit pattern. The image will be simplified and mapped to commit counts."
      startDate={startDate}
      data={data}
      setDataAtIndex={setDataAtIndex}
    >
      <InputDate legend="Start date" date={startDate} onDateChange={onStartDateChange} dateErr={startDateErr} />
      <InputFile legend="Image" accept="image/*" onChange={onChange} />
    </PatternTab>
  );
}

export default Image;
