import * as React from "react";

import { CommitCount } from "../constants.ts";
import type { OnButtonClick, SetDataAtIndexFunc } from "../types.ts";
import Preview from "./Preview.tsx";

type Prop = {
  title: string;
  subtitle: React.ReactNode;
  children: React.ReactNode;
  startDate: string;
  data: CommitCount[];
  setDataAtIndex: SetDataAtIndexFunc;
  isGenerating?: boolean;
  onGenerate?: OnButtonClick;
};

function PatternInput({ title, subtitle, children, startDate, data, setDataAtIndex, isGenerating, onGenerate }: Prop) {
  const disableGenerateCls = isGenerating || !onGenerate ? "btn-disabled" : "";
  return (
    <div className="card bg-neutral text-neutral-content shadow-sm">
      <div className="card-body">
        <h2 className="card-title">{title}</h2>
        <p>{subtitle}</p>
        <div className="my-4 grid gap-x-4 gap-y-1 grid-cols-1 md:grid-cols-2">{children}</div>
        <Preview startDate={startDate} data={data} setDataAtIndex={setDataAtIndex} />
        <div className="card-actions justify-center">
          <button className={`btn btn-secondary ${disableGenerateCls}`} onClick={onGenerate}>
            {isGenerating ? <span className="loading loading-spinner" /> : null}
            Generate Repo
          </button>
        </div>
      </div>
    </div>
  );
}

export default PatternInput;
