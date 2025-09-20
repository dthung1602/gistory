import type { Dispatch } from "react";

import { type CommitCount, Font, VisualizerMethod } from "./constants.ts";
import type { AddToast } from "./context.tsx";
import type { CreateRepoData } from "./types.ts";

const BACKEND_ENDPOINT = import.meta.env.VITE_BACKEND_ENDPOINT || "/api/";

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

function createRepo(repo: CreateRepoData): ReqControl {
  const data = {
    ...repo,
    visualizer_method: {
      method: "RawPattern",
      start_date: repo.startDate,
      raw_pattern: repo.data,
    },
  };
  return post("repo", data);
}

function getRepo(uuid: string): ReqControl {
  return get(`repo/${uuid}`);
}

function genDownloadUrl(uuid: string, name: string) {
  return `${BACKEND_ENDPOINT}download/${uuid}/${name}.tar.zst`;
}

function download(url: string) {
  const link = document.createElement("a");
  link.href = url;
  link.target = "_blank";
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

type ReqControl = [Promise<Response>, AbortController];

function get(path: string, req: Record<string, unknown> = {}): ReqControl {
  const params = new URLSearchParams();
  const controller = new AbortController();
  for (const [k, v] of Object.entries(req)) {
    params.append(k, "" + v);
  }
  const url = BACKEND_ENDPOINT + path + "?" + params;
  const fetchPromise = fetch(url, { signal: controller.signal }).then(checkStatus);
  return [fetchPromise, controller];
}

function post(path: string, req: Record<string, unknown>): ReqControl {
  const controller = new AbortController();
  const url = BACKEND_ENDPOINT + path;
  const fetchPromise = fetch(url, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req),
    signal: controller.signal,
  }).then(checkStatus);
  return [fetchPromise, controller];
}

async function checkStatus(res: Response) {
  if (!res.ok) {
    const body = await res.text();
    let message = `Response with status ${res.status} ${res.statusText}. Body: ${body}`;
    console.error(message);
    message = message.slice(0, 512); // keep it short
    throw Error(message);
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

export default { preview, upload, errHandler, createRepo, getRepo, genDownloadUrl, download };
