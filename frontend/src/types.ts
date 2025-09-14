import type { ChangeEvent, JSX, MouseEvent } from "react";

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

type CreateRepoData = {
  name: string;
  username: string;
  email: string;
  branch: string;
  timezone: string;
  startDate: string;
  data: CommitCount[];
};

type UpdateCreateRepoData = (newData: Partial<CreateRepoData>) => void;

type UpdatePreviewData = (data: CommitCount[]) => void;
type SelectPatternTabProp = { updatePreviewData: UpdatePreviewData };

type TabComponent = (prop: SelectPatternTabProp & { key: string }) => JSX.Element;
type TabId = "Daily" | "Random" | "TextFilePattern" | "Image" | "Text";
type TabSetting = { tabLabel: string; componentClass: TabComponent; defaultChecked?: boolean };

export type {
  OnInputChange,
  OnSelectChange,
  SelectInputProp,
  SetDataAtIndexFunc,
  OnButtonClick,
  FileUploadResult,
  PreviewResult,
  CreateRepoData,
  UpdateCreateRepoData,
  UpdatePreviewData,
  SelectPatternTabProp,
  TabComponent,
  TabId,
  TabSetting,
};
