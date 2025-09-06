import type { ChangeEvent } from "react";

import type { CommitCount } from "./constants.ts";

type OnInputChange = (event: ChangeEvent<HTMLInputElement>) => void;
type OnSelectChange = (event: ChangeEvent<HTMLSelectElement>) => void;
type SelectInputProp<V = string> = {
  value: V;
  onChange: OnSelectChange;
};
type SetDataAtIndexFunc = (c: CommitCount, i: number) => void;
export type { OnInputChange, OnSelectChange, SelectInputProp, SetDataAtIndexFunc };
