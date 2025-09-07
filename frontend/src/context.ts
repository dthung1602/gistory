import { createContext } from "react";
import * as React from "react";

type ToastData = {
  key: number;
  type: "success" | "info" | "warning" | "error";
  content: React.ReactNode;
};

type AddToast = (toast: ToastData, displayTime?: number) => void;

type ToastContextState = {
  toasts: ToastData[];
  addToast: AddToast;
};

const dummyToastContext = {
  toasts: [],
  addToast: () => {
    console.log("XXX");
  },
};
const ToastContext = createContext<ToastContextState>(dummyToastContext);

export { ToastContext, type AddToast, type ToastData, type ToastContextState };
