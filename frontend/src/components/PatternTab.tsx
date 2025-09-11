import * as React from "react";

import { CommitCount } from "../constants.ts";
import type { SetDataAtIndexFunc } from "../types.ts";
import Preview from "./Preview.tsx";

type Prop = {
  title: string;
  subtitle: React.ReactNode;
  children: React.ReactNode;
  startDate: string;
  data: CommitCount[];
  setDataAtIndex: SetDataAtIndexFunc;
  loading?: boolean;
};

function PatternTab({ title, subtitle, children, startDate, data, setDataAtIndex, loading = false }: Prop) {
  return (
    <div className="card bg-neutral text-neutral-content shadow-sm">
      <div className="card-body">
        <h2 className="card-title">{title}</h2>
        <p>{subtitle}</p>
        <div className="my-4 grid gap-x-4 gap-y-1 grid-cols-1 md:grid-cols-2">{children}</div>
        <Preview startDate={startDate} data={data} setDataAtIndex={setDataAtIndex} loading={loading} />
      </div>
    </div>
  );
}

export default PatternTab;
