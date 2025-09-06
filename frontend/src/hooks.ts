import { type ChangeEvent, type RefObject, useCallback, useEffect, useRef, useState } from "react";

import { CommitCount, Font, SUNDAY } from "./constants.ts";
import type { OnInputChange, OnSelectChange, SetDataAtIndexFunc } from "./types.ts";
import { debounce } from "./utils.ts";

type UseDateInputArg = {
  minDate?: string;
  mustBeSunday?: boolean;
};

function useDateInput({ minDate = "0", mustBeSunday = false }: UseDateInputArg = {}): [string, OnInputChange, string] {
  const [date, setDate] = useState<string>(new Date().toDateString());
  const [error, setError] = useState<string>("");

  const onChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      setDate(event.target.value);
    },
    [setDate],
  );

  useEffect(() => {
    if (date == "") {
      return;
    }
    let err = "";
    if (date < minDate) {
      err = "End date must come after start date. ";
    }
    if (mustBeSunday && new Date(date).getDay() != SUNDAY) {
      err += "Must be a Sunday. ";
    }
    setError(err);
  }, [minDate, mustBeSunday, date]);

  return [date, onChange, error];
}

function useEnumInput<E extends string>(enumObj: Record<string, E>, defaultValue: E): [E, OnSelectChange] {
  const [value, setValue] = useState<E>(defaultValue);

  const onChange = useCallback(
    (event: ChangeEvent<HTMLSelectElement>) => {
      const val = event.target.value;
      if (val in enumObj) {
        setValue(enumObj[val]);
      } else {
        console.error("Invalid value: " + val);
      }
    },
    [setValue, enumObj],
  );

  return [value, onChange];
}

function useCommitCountInput(): [CommitCount, OnSelectChange] {
  return useEnumInput<CommitCount>(CommitCount, CommitCount.Zero);
}

function useFontInput(): [Font, OnSelectChange] {
  return useEnumInput<Font>(Font, Font.SubwayTracker);
}

function useFileInput(): [File | null, OnInputChange] {
  const [file, setFile] = useState<File | null>(null);
  const onChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      if (event.target.files != null) {
        setFile(event.target.files.item(0));
      }
    },
    [setFile],
  );
  return [file, onChange];
}

function useTextInput(debounceTime: number = 0): [string, OnInputChange, RefObject<HTMLInputElement | null>] {
  const [text, setText] = useState("");
  const ref = useRef<HTMLInputElement>(null);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const onChange = useCallback(
    debounce(() => {
      if (ref.current != null) {
        setText(ref.current.value);
      }
    }, debounceTime),
    [setText, ref],
  );
  return [text, onChange, ref];
}

function usePreviewData(): [CommitCount[], SetDataAtIndexFunc] {
  const [data, setData] = useState<CommitCount[]>(mockData);

  const setDataAtIndex = useCallback(
    (c: CommitCount, idx: number) => {
      const newData = [...data];
      newData[idx] = c;
      setData(newData);
    },
    [data],
  );

  return [data, setDataAtIndex];
}

let mockData = [
  CommitCount.ALot,
  CommitCount.ALot,
  CommitCount.Many,
  CommitCount.Many,
  CommitCount.Zero,
  CommitCount.Few,
  CommitCount.Some,
  //
  CommitCount.Some,
  CommitCount.Zero,
  CommitCount.ALot,
  CommitCount.Few,
  CommitCount.Zero,
  CommitCount.Some,
  CommitCount.ALot,
  //
  CommitCount.Some,
  CommitCount.ALot,
  CommitCount.Many,
  CommitCount.Zero,
  CommitCount.Some,
  CommitCount.Few,
  CommitCount.Many,
  //
  CommitCount.Zero,
  CommitCount.Some,
  CommitCount.ALot,
  CommitCount.Many,
];

mockData = [...mockData, ...mockData, ...mockData, ...mockData, ...mockData];
mockData = [...mockData, ...mockData];

export { useDateInput, useCommitCountInput, useFontInput, useFileInput, useTextInput, usePreviewData };
