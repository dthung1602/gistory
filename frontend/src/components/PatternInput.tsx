import * as React from "react";

import { CommitCount } from "../constants.ts";
import Preview from "./Preview.tsx";

type Prop = {
  title: string;
  subtitle: React.ReactNode;
  children: React.ReactNode;
};

const today = new Date();

function PatternInput({ title, subtitle, children }: Prop) {
  return (
    <div className="card bg-neutral text-neutral-content shadow-sm">
      <div className="card-body">
        <h2 className="card-title">{title}</h2>
        <p>{subtitle}</p>
        <div className="my-4 grid gap-x-4 gap-y-1 grid-cols-1 md:grid-cols-2">{children}</div>
        <Preview startDate={today} data={mockData} />
        <div className="card-actions justify-center">
          <button className="btn btn-secondary">Generate Repo</button>
        </div>
      </div>
    </div>
  );
}

let mockData = [
  CommitCount.ALot,
  CommitCount.ALot,
  CommitCount.Many,
  CommitCount.Many,
  CommitCount.Zero,
  CommitCount.Few,
  CommitCount.Some,
  //
  CommitCount.Some,
  CommitCount.Zero,
  CommitCount.ALot,
  CommitCount.Few,
  CommitCount.Zero,
  CommitCount.Some,
  CommitCount.ALot,
  //
  CommitCount.Some,
  CommitCount.ALot,
  CommitCount.Many,
  CommitCount.Zero,
  CommitCount.Some,
  CommitCount.Few,
  CommitCount.Many,
  //
  CommitCount.Zero,
  CommitCount.Some,
  CommitCount.ALot,
  CommitCount.Many,
];

mockData = [...mockData, ...mockData, ...mockData, ...mockData, ...mockData];
mockData = [...mockData, ...mockData];

export default PatternInput;
