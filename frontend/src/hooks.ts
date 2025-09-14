import {
  type ChangeEvent,
  type Dispatch,
  type RefObject,
  type SetStateAction,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";

import { CommitCount, Font, SUNDAY, TODAY_STR } from "./constants.ts";
import { FormContext } from "./context.tsx";
import type { OnInputChange, OnSelectChange, SetDataAtIndexFunc, UpdatePreviewData } from "./types.ts";
import { debounce } from "./utils.ts";

type UseDateInputArg = {
  minDate?: string;
  mustBeSunday?: boolean;
  defaultValue?: string;
};
type UseDateInputReturn = [string, OnInputChange, string];

function useDateInput({
  minDate = "0",
  mustBeSunday = false,
  defaultValue = TODAY_STR,
}: UseDateInputArg = {}): UseDateInputReturn {
  const [date, setDate] = useState<string>(defaultValue);
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

function useStartDateInput({
  minDate = "0",
  mustBeSunday = false,
  defaultValue = TODAY_STR,
}: UseDateInputArg = {}): UseDateInputReturn {
  const { updateFormContext } = useContext(FormContext);
  const value = useDateInput({ minDate, mustBeSunday, defaultValue });

  useEffect(() => {
    updateFormContext({ startDate: value[0] });
  }, [value[0], updateFormContext]);

  return value;
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
  return useEnumInput<CommitCount>(CommitCount, CommitCount.Some);
}

function useFontInput(): [Font, OnSelectChange] {
  return useEnumInput<Font>(Font, Font.SubwayTracker);
}

type UseFileInput = {
  file: File | null;
  onChange: OnInputChange;
  fileId: string;
  setFileId: Dispatch<string>;
  fileErr: string;
  setFileErr: Dispatch<string>;
};

function useFileInput(): UseFileInput {
  const [file, setFile] = useState<File | null>(null);
  const [fileId, setFileId] = useState("");
  const [fileErr, setFileErr] = useState("");

  const onChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      if (event.target.files != null) {
        setFile(event.target.files.item(0));
      }
    },
    [setFile],
  );
  return { file, onChange, fileId, setFileId, fileErr, setFileErr };
}

type ValidatorFunc = (val: string) => string;

type UseTextInputArg = {
  debounceTime?: number;
  validator?: ValidatorFunc;
};

type UseTextInputReturn = [string, string, OnInputChange, RefObject<HTMLInputElement | null>];

function useTextInput({ debounceTime = 0, validator }: UseTextInputArg): UseTextInputReturn {
  const [text, setText] = useState("");
  const [error, setError] = useState<string>("");

  const ref = useRef<HTMLInputElement>(null);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const onChange = useCallback(
    debounce(() => {
      if (ref.current != null) {
        setText(ref.current.value);
        if (validator) {
          setError(validator(ref.current.value));
        }
      }
    }, debounceTime),
    [setText, ref],
  );

  return [text, error, onChange, ref];
}

type UseInputHookNoValidatorArg = {
  debounceTime?: number;
  regex: RegExp;
  errorMessage: string;
};

function useTextInputRegex({ regex, debounceTime = 0, errorMessage }: UseInputHookNoValidatorArg): UseTextInputReturn {
  const [text, setText] = useState("");
  const [error, setError] = useState<string>("");

  const ref = useRef<HTMLInputElement>(null);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const onChange = useCallback(
    debounce(() => {
      if (ref.current != null) {
        setText(ref.current.value);
        setError(ref.current.value.match(regex) ? "" : errorMessage);
      }
    }, debounceTime),
    [setText, ref],
  );

  return [text, error, onChange, ref];
}

const EMAIL_INPUT_ARGS: UseInputHookNoValidatorArg = {
  regex: /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
  debounceTime: 350,
  errorMessage: "Invalid email address",
};

function useEmailInput(): UseTextInputReturn {
  const ret = useTextInputRegex(EMAIL_INPUT_ARGS);
  return ret;
}

const TIMEZONE_INPUT_ARGS: UseInputHookNoValidatorArg = {
  regex: /^[+-][0-9]{4}$/,
  debounceTime: 50,
  errorMessage: "Timezone must have format like +0700 -0430",
};

function useTimezoneInput(): UseTextInputReturn {
  const ret = useTextInputRegex(TIMEZONE_INPUT_ARGS);
  return ret;
}

const GITHUB_NAME_INPUT_ARGS: UseInputHookNoValidatorArg = {
  regex: /^[a-zA-Z0-9-._]{1,64}$/,
  debounceTime: 50,
  errorMessage: 'Alphanumeric and "-._". No more than 64 chars',
};

function useGithubName(): UseTextInputReturn {
  return useTextInputRegex(GITHUB_NAME_INPUT_ARGS);
}

const GITHUB_BRANCH_INPUT_ARGS: UseInputHookNoValidatorArg = {
  regex: /^[a-zA-Z0-9-._/]{1,64}$/,
  debounceTime: 50,
  errorMessage: 'Alphanumeric and "-._/". No more than 64 chars',
};

function useGithubBranch(): UseTextInputReturn {
  return useTextInputRegex(GITHUB_BRANCH_INPUT_ARGS);
}

function usePreviewData(
  updateData: UpdatePreviewData,
): [CommitCount[], Dispatch<SetStateAction<CommitCount[]>>, SetDataAtIndexFunc] {
  const [data, setData] = useState<CommitCount[]>(mockData);

  const setDataAtIndex = useCallback(
    (c: CommitCount, idx: number) => {
      const newData = [...data];
      newData[idx] = c;
      setData(newData);
    },
    [data],
  );

  useEffect(() => updateData(data), [data, updateData]);

  return [data, setData, setDataAtIndex];
}

const mockData = Array(180).fill(CommitCount.Some);

export {
  useDateInput,
  useStartDateInput,
  useCommitCountInput,
  useFontInput,
  useFileInput,
  useTextInput,
  useEmailInput,
  useTimezoneInput,
  useGithubName,
  useGithubBranch,
  usePreviewData,
};
