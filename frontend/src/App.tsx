import Footer from "./components/Footer.tsx";
import Header from "./components/Header.tsx";
import SelectPattern from "./components/SelectPattern.tsx";
import Toasts from "./components/Toasts.tsx";
import { ToastContext, useToastContextState } from "./context.ts";

function App() {
  const { toasts, addToast } = useToastContextState();

  return (
    <ToastContext.Provider value={{ toasts, addToast }}>
      <Header />
      <SelectPattern />
      <Toasts />
      <Footer />
    </ToastContext.Provider>
  );
}

export default App;
