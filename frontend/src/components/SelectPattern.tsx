import { type JSX, useState } from "react";

import Daily from "./Daily.tsx";
import Image from "./Image.tsx";
import Random from "./Random.tsx";
import Text from "./Text.tsx";
import TextFilePattern from "./TextFilePattern.tsx";

type TabComponent = (prop: { key: string }) => JSX.Element;
type TabId = "Daily" | "Random" | "TextFilePattern" | "Image" | "Text";
type TabSetting = { tabLabel: string; componentClass: TabComponent; defaultChecked?: boolean };

const TABS: Record<TabId, TabSetting> = {
  Daily: { tabLabel: "Daily", componentClass: Daily, defaultChecked: true },
  Random: { tabLabel: "Random", componentClass: Random },
  TextFilePattern: { tabLabel: "Text File Pattern", componentClass: TextFilePattern },
  Image: { tabLabel: "Image", componentClass: Image },
  Text: { tabLabel: "Text", componentClass: Text },
};

function SelectPattern() {
  const [tab, setTab] = useState<TabId>("Daily");
  const handleTabClick = (newTab: TabId) => () => setTab(newTab);

  return (
    <div className="w-full flex justify-center p-4">
      <div className="">
        <div className="text-xl font-bold px-1 py-4">Select pattern type</div>
        <div className="tabs tabs-box bg-base-100">
          {Object.entries(TABS).map(([tabId, { tabLabel, componentClass, defaultChecked = false }]) => [
            <input
              key={`tab-label-${tabLabel}`}
              type="radio"
              name="method"
              className="tab [--tab-bg:var(--color-primary)] text-primary-content"
              aria-label={tabLabel}
              defaultChecked={defaultChecked}
              onClick={handleTabClick(tabId as TabId)}
            />,
            <div key={`tab-content-${tabLabel}`} className="tab-content py-6">
              {componentClass({ key: tabLabel })}
            </div>,
          ])}
        </div>
      </div>
    </div>
  );
}

export default SelectPattern;
