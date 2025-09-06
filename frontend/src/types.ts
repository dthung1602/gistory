import type { ChangeEvent } from "react";

type OnInputChange = (event: ChangeEvent<HTMLInputElement>) => void;
type OnSelectChange = (event: ChangeEvent<HTMLSelectElement>) => void;
type SelectInputProp<V = string> = {
  value: V;
  onChange: OnSelectChange;
};

export type { OnInputChange, OnSelectChange, SelectInputProp };
