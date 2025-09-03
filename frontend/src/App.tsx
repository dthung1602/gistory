import Daily from "./components/Daily.tsx";
import Random from "./components/Random.tsx";
import Manual from "./components/Manual.tsx";
import TextFilePattern from "./components/TextFilePattern.tsx";
import Image from "./components/Image.tsx";
import Text from "./components/Text.tsx";

const tabs = [
  { name: "Daily", componentClass: Daily },
  { name: "Random", componentClass: Random },
  { name: "Manual Input", componentClass: Manual },
  { name: "Text File Pattern", componentClass: TextFilePattern },
  { name: "Image", componentClass: Image },
  { name: "Text", componentClass: Text },
];

function App() {
  const elements = [];
  for (const { name, componentClass } of tabs) {
    elements.push(
      <input
        key={`tab-label-${name}`}
        type="radio"
        name="method"
        className="tab [--tab-bg:var(--color-primary)] text-primary-content"
        aria-label={name}
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
      <div className="hero my-8">
        <div className="hero-content text-center">
          <div className="max-w-xxl">
            <h1 className="text-7xl font-bold">Gistory</h1>
            <p className="text-2xl italic py-6">
              Create custom commit patterns to display on your GitHub profile
            </p>
          </div>
        </div>
      </div>
      <div className="w-full flex justify-center p-4">
        <div className="max-w-[900px]">
          <div className="text-xl font-bold px-1 py-4">Select pattern type</div>
          <div className="tabs tabs-box bg-base-100">{elements}</div>
        </div>
      </div>
    </>
  );
}

export default App;
