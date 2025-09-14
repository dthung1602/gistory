import type { Dispatch } from "react";

import { type CommitCount, Font, VisualizerMethod } from "./constants.ts";
import type { AddToast } from "./context.tsx";

const BACKEND_ENDPOINT = import.meta.env.BACKEND_ENDPOINT || "http://localhost:5173/api/";

type PreviewReq = {
  method: VisualizerMethod;
  start_date: string;
  end_date?: string;
  commit_count?: CommitCount;
  font?: Font;
  input_file?: string;
  text?: string;
};

function preview(req: PreviewReq): ReqControl {
  return get("preview", req);
}

function upload(file: File): ReqControl {
  const controller = new AbortController();
  const formData = new FormData();
  formData.append("file", file);

  const url = BACKEND_ENDPOINT + "upload";
  const promise = fetch(url, {
    method: "POST",
    body: formData,
    signal: controller.signal,
  }).then(checkStatus);

  return [promise, controller];
}

type ReqControl = [Promise<Response>, AbortController];

function get(path: string, req: Record<string, unknown>): ReqControl {
  const params = new URLSearchParams();
  const controller = new AbortController();
  for (const [k, v] of Object.entries(req)) {
    params.append(k, "" + v);
  }
  const url = BACKEND_ENDPOINT + path + "?" + params;
  const fetchPromise = fetch(url, { signal: controller.signal }).then(checkStatus);
  return [fetchPromise, controller];
}

function checkStatus(res: Response) {
  if (!res.ok) {
    console.error(`Response with status ${res.status} ${res.statusText}`);
    throw Error(res.statusText);
  }
  return res;
}

function errHandler(addToast: AddToast, setErr: Dispatch<string> = () => {}) {
  return (err: Error) => {
    if (err.name == "AbortError") {
      return;
    }
    const errStr = err + "";
    setErr(errStr);
    addToast({
      key: Math.random(),
      type: "error",
      content: (
        <span>
          <b>Error:</b> {errStr}
        </span>
      ),
    });
  };
}

export default { preview, upload, errHandler, checkStatus };
