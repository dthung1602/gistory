import { type ChangeEvent, type MouseEvent } from "react";

import type { CommitCount } from "./constants.ts";

type OnInputChange = (event: ChangeEvent<HTMLInputElement>) => void;
type OnSelectChange = (event: ChangeEvent<HTMLSelectElement>) => void;
type OnButtonClick = (event: MouseEvent<HTMLButtonElement>) => void;

type SelectInputProp<V = string> = {
  value: V;
  onChange: OnSelectChange;
};
type SetDataAtIndexFunc = (c: CommitCount, i: number) => void;

type FileUploadResult = {
  uuid: string;
  content_type: string;
  size: number;
};

type PreviewResult = {
  data: CommitCount[];
};

export type {
  OnInputChange,
  OnSelectChange,
  SelectInputProp,
  SetDataAtIndexFunc,
  OnButtonClick,
  FileUploadResult,
  PreviewResult,
};
