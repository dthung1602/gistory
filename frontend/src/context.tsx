import React, { type JSX, createContext, useCallback, useMemo, useState } from "react";

import type { CommitCount } from "./constants.ts";

//-------------------------------
// Toast Context
//-------------------------------

type ToastData = {
  key?: number;
  type: "success" | "info" | "warning" | "error";
  content: React.ReactNode;
};

type AddToast = (toast: ToastData, displayTime?: number) => void;

type ToastContextState = {
  toasts: ToastData[];
  addToast: AddToast;
};

const ToastContext = createContext<ToastContextState>({
  toasts: [],
  addToast: () => {},
});

function ToastContextProvider({ children }: { children: React.ReactNode }): JSX.Element {
  const [toasts, setToasts] = useState<ToastData[]>([]);

  const addToast: AddToast = useCallback(
    (toast, displayTime = 5000) => {
      if (toast.key === undefined) toast.key = Math.random();
      setToasts(prev => [toast, ...prev]);
      const notThisToast = (t: ToastData) => t !== toast;
      setTimeout(() => setToasts(prev => [...prev.filter(notThisToast)]), displayTime);
    },
    [setToasts],
  );

  const contextValue = useMemo(
    () => ({
      toasts,
      addToast,
    }),
    [toasts, addToast],
  );

  return <ToastContext value={contextValue}>{children}</ToastContext>;
}

//-------------------------------
// Create Repo Context
//-------------------------------
type UpdateFormContext = (newData: Partial<FormContextState>) => void;

type FormContextStateData = {
  name: string | null;
  username: string | null;
  email: string | null;
  branch: string | null;
  timezone: string | null;
  startDate: string | null;
  data: CommitCount[];
  inputErrors: Record<string, boolean>;
};

type FormContextState = FormContextStateData & {
  updateFormContext: UpdateFormContext;
};

const EMPTY_FORM_CONTEXT_DATA: FormContextStateData = {
  name: null,
  username: null,
  email: null,
  branch: null,
  timezone: null,
  startDate: null,
  data: [],
  inputErrors: {},
};

const FormContext = createContext<FormContextState>({
  ...EMPTY_FORM_CONTEXT_DATA,
  updateFormContext: () => {},
});

function FormContextProvider({ children }: { children: React.ReactNode }): JSX.Element {
  const [formData, setFormData] = useState<FormContextStateData>(EMPTY_FORM_CONTEXT_DATA);

  const updateFormContext: UpdateFormContext = useCallback(
    updated => {
      setFormData(prev => {
        const newInputErrors =
          updated.inputErrors !== undefined ? { ...prev.inputErrors, ...updated.inputErrors } : prev.inputErrors;
        return { ...prev, ...updated, inputErrors: newInputErrors };
      });
    },
    [setFormData],
  );

  const contextValue: FormContextState = useMemo(
    () => ({ ...formData, updateFormContext }),
    [formData, updateFormContext],
  );

  return <FormContext value={contextValue}>{children}</FormContext>;
}

export { ToastContext, ToastContextProvider, FormContext, FormContextProvider };
export type { ToastData, ToastContextState, AddToast };
