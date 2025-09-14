import CreateRepo from "./components/CreateRepo.tsx";
import Footer from "./components/Footer.tsx";
import Header from "./components/Header.tsx";
import Toasts from "./components/Toasts.tsx";
import { ToastContextProvider } from "./context.tsx";

function App() {
  return (
    <ToastContextProvider>
      <Header />
      <CreateRepo />
      <Toasts />
      <Footer />
    </ToastContextProvider>
  );
}

export default App;
