import { useContext, useEffect, useMemo, useState } from "react";

import type { CommitCount } from "../constants.ts";
import { FormContext } from "../context.tsx";
import type { TabId, TabSetting } from "../types.ts";
import Daily from "./Daily.tsx";
import Image from "./Image.tsx";
import Random from "./Random.tsx";
import Text from "./Text.tsx";
import TextFilePattern from "./TextFilePattern.tsx";

const TABS: Record<TabId, TabSetting> = {
  Daily: { tabLabel: "Daily", componentClass: Daily, defaultChecked: true },
  Random: { tabLabel: "Random", componentClass: Random },
  TextFilePattern: { tabLabel: "Text File Pattern", componentClass: TextFilePattern },
  Image: { tabLabel: "Image", componentClass: Image },
  Text: { tabLabel: "Text", componentClass: Text },
};

function SelectPattern() {
  const { updateFormContext } = useContext(FormContext);
  const [tab, setTab] = useState<TabId>("Daily");
  const [tabsData, setTabsData] = useState<Record<TabId, CommitCount[]>>({
    Daily: [],
    Random: [],
    TextFilePattern: [],
    Image: [],
    Text: [],
  });

  const [handleTabClickFuncs, setTabDataFuncs] = useMemo(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    const handleTabClickFuncs: Record<TabId, () => void> = {};
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    const setTabDataFuncs: Record<TabId, (data: CommitCount[]) => void> = {};

    for (const tabId in TABS) {
      handleTabClickFuncs[tabId as TabId] = () => {
        setTab(tabId as TabId);
      };
      setTabDataFuncs[tabId as TabId] = (data: CommitCount[]) => {
        setTabsData(prev => ({ ...prev, [tabId as TabId]: data }));
      };
    }
    return [handleTabClickFuncs, setTabDataFuncs];
  }, []);

  useEffect(() => {
    updateFormContext({ data: tabsData[tab] });
  }, [tab, tabsData, updateFormContext]);

  return (
    <>
      <div className="text-xl font-bold px-1 py-4">Select pattern type</div>
      <div className="tabs tabs-box bg-base-100 p-0">
        {Object.entries(TABS).map(([tabId, { tabLabel, componentClass, defaultChecked = false }]) => [
          <input
            key={`tab-label-${tabLabel}`}
            type="radio"
            name="method"
            className="tab [--tab-bg:var(--color-primary)] text-primary-content"
            aria-label={tabLabel}
            defaultChecked={defaultChecked}
            onClick={handleTabClickFuncs[tabId as TabId]}
          />,
          <div key={`tab-content-${tabLabel}`} className="tab-content py-6">
            {componentClass({ key: tabLabel, updatePreviewData: setTabDataFuncs[tabId as TabId] })}
          </div>,
        ])}
      </div>
    </>
  );
}

export default SelectPattern;
