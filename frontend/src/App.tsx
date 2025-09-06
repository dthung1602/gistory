import Daily from "./components/Daily.tsx";
import Footer from "./components/Footer.tsx";
import Header from "./components/Header.tsx";
import Image from "./components/Image.tsx";
import Manual from "./components/Manual.tsx";
import Random from "./components/Random.tsx";
import Text from "./components/Text.tsx";
import TextFilePattern from "./components/TextFilePattern.tsx";

const tabs = [
  { name: "Daily", componentClass: Daily, defaultChecked: true },
  { name: "Random", componentClass: Random },
  { name: "Text File Pattern", componentClass: TextFilePattern },
  { name: "Image", componentClass: Image },
  { name: "Text", componentClass: Text },
  { name: "Manual Input", componentClass: Manual },
];

function App() {
  const elements = [];
  for (const { name, componentClass, defaultChecked } of tabs) {
    elements.push(
      <input
        key={`tab-label-${name}`}
        type="radio"
        name="method"
        className="tab [--tab-bg:var(--color-primary)] text-primary-content"
        aria-label={name}
        defaultChecked={defaultChecked}
      />,
    );
    elements.push(
      <div key={`tab-content-${name}`} className="tab-content py-6">
        {componentClass()}
      </div>,
    );
  }

  return (
    <>
      <Header />
      <div className="w-full flex justify-center p-4">
        <div className="max-w-[900px]">
          <div className="text-xl font-bold px-1 py-4">Select pattern type</div>
          <div className="tabs tabs-box bg-base-100">{elements}</div>
        </div>
      </div>
      <Footer />
    </>
  );
}

export default App;
