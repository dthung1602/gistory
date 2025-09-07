import * as React from "react";
import { createContext, useCallback, useMemo, useState } from "react";

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

const ToastContext = createContext<ToastContextState>({
  toasts: [],
  addToast: () => {},
});

function useToastContextState(): ToastContextState {
  const [toasts, setToasts] = useState<ToastData[]>([]);

  const addToast: AddToast = useCallback(
    (toast, displayTime = 5000) => {
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

  return contextValue;
}

export { ToastContext, useToastContextState, type ToastData, type ToastContextState, type AddToast };
