import * as React from "react";
import { useMemo } from "react";

import { CommitCount } from "../constants.ts";
import { dateFormater, monthFormater, nextCommitCount } from "../utils.ts";

type Prop = {
  readonly startDate: string;
  readonly data: CommitCount[];
  readonly setDataAtIndex: (commitCount: CommitCount, idx: number) => void;
};

type DataCell = {
  key: string;
  cellLabel: string;
  commitCount: CommitCount | null;
  onClick: (event: React.MouseEvent<HTMLDivElement>) => void;
};
type LabelCell = {
  key: string;
  text: string;
};
type Cell = LabelCell | DataCell;

const COMMIT_COUNT_VALS = [null, ...Object.values(CommitCount)];

function getColorForCommit(count: CommitCount | null): string {
  switch (count) {
    case CommitCount.Zero:
      return "bg-gray-200";
    case CommitCount.Few:
      return "bg-green-100";
    case CommitCount.Some:
      return "bg-green-300";
    case CommitCount.Many:
      return "bg-green-500";
    case CommitCount.ALot:
      return "bg-green-700";
    default:
      return "bg-transparent";
  }
}

function addEmptyDataCells(cells: Cell[], n: number) {
  for (let i = 0; i < n; i++) {
    cells.push({
      cellLabel: "No commit",
      commitCount: null,
      onClick: () => {},
      key: "" + Math.random(),
    });
  }
}

function Preview({ startDate, data, setDataAtIndex }: Prop) {
  const cells = useMemo(() => {
    const date = new Date(startDate);
    const newCells: Cell[] = [];

    // Ensure we have at least 7 days per week representation
    // Add empty cells at the beginning
    newCells.push({ text: "", key: "" + Math.random() });
    addEmptyDataCells(newCells, date.getDay());

    const initEmptyCellCount = newCells.length;

    let lastMonth = -1;
    for (let i = 0; i < data.length; i++) {
      // Beginning of new column.
      // Add month label if this column belongs to a different month
      if ((i + initEmptyCellCount) % 7 == 1) {
        const currentMonth = date.getMonth();
        let text = "";
        // The labels shouldn't be too close together,
        // hence we do not add a label if date in month > 16
        if (currentMonth != lastMonth && date.getDate() < 16) {
          text = monthFormater.format(date);
          lastMonth = currentMonth;
        }
        // remove redundant padding if start date is a sunday
        if (i == 0) {
          newCells.pop();
        }
        newCells.push({
          text,
          key: "label-" + date.toISOString(),
        });
      }

      const commitCount = data[i];
      const cellLabel: string = `${commitCount} commits on ${dateFormater.format(date)}`;
      const onClick = () => {
        setDataAtIndex(nextCommitCount(data[i]), i);
      };
      const key = "data-" + date.toISOString();
      newCells.push({ cellLabel, commitCount, onClick, key });

      date.setDate(date.getDate() + 1);
    }

    // Add empty cells at the end
    addEmptyDataCells(newCells, (7 - date.getDay()) % 7);

    return newCells;
  }, [startDate, data, setDataAtIndex]);

  return (
    <div className="mb-4 flex flex-col items-start">
      <h2 className="font-bold text-lg mb-2">Preview</h2>
      <div className="pb-4 pt-3 pl-8 mb-2 graph-container overflow-x-scroll">
        <div className="grid grid-rows-8 grid-flow-col gap-1.5 justify-start" aria-label="commit-graph">
          <div key="First" />
          <div key="Sun" />
          <div key="Mon" className="mr-2 font-bold">
            Mon
          </div>
          <div key="Tue" />
          <div key="Wed" className="mr-2 font-bold">
            Wed
          </div>
          <div key="Thu" />
          <div key="Fri" className="mr-2 font-bold">
            Fri
          </div>
          <div key="Sat" />
          {cells.map(cell => {
            if ("text" in cell) {
              return (
                <div key={cell.key} className="w-5 h-5 font-bold whitespace-nowrap">
                  {cell.text}
                </div>
              );
            }

            const isFilled = cell.commitCount !== null;
            const colorClass = getColorForCommit(cell.commitCount);
            const borderClass = isFilled ? "border border-black/0" : "border";
            return (
              <div
                key={cell.key}
                onClick={cell.onClick}
                className={`w-5 h-5 rounded-xs tooltip tooltip-info foo hover:scale-110 hover:z-10
                      cursor-pointer transition-colors duration-250 ${colorClass} ${borderClass}`}
                data-tip={cell.cellLabel}
              />
            );
          })}
        </div>
      </div>
      <div className="flex justify-between w-full text-info">
        <a
          className="hover:underline"
          href="https://docs.github.com/articles/why-are-my-contributions-not-showing-up-on-my-profile"
        >
          How GitHub calculate contribution?
        </a>
        <div className="gap-1 hidden sm:flex">
          Less &nbsp;
          {COMMIT_COUNT_VALS.map(commit => (
            <div key={"" + commit} className={`w-5 h-5 inline-block rounded-sm border ${getColorForCommit(commit)}`} />
          ))}
          &nbsp; More
        </div>
      </div>
    </div>
  );
}

export default Preview;
