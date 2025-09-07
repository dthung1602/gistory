import { type CommitCount, Font, VisualizerMethod } from "./constants.ts";

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
  });

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
  return [fetch(url, { signal: controller.signal }), controller];
}

export default { preview, upload };
