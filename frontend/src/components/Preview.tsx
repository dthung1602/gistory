import { CommitCount } from "../constants.ts";
import { useMemo } from "react";
import { isString } from "../utils.ts";

type Prop = {
  startDate: Date;
  data: CommitCount[];
};

type Cell = string | { cellLabel: string; commitCount: CommitCount };

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

function Preview({ startDate, data }: Prop) {
  // Ensure we have at least 7 days per week representation
  const cells = useMemo(() => {
    const totalDataCells = Math.ceil(data.length / 7) * 7;

    const cells: Cell[] = [];

    let lastMonth = -1;
    const dateFormater = new Intl.DateTimeFormat("en-US", {
      month: "short",
      day: "numeric",
    });
    const monthFormater = new Intl.DateTimeFormat("en-US", { month: "short" });
    const date = startDate;

    for (let i = 0; i < totalDataCells; i++) {
      const currentMonth = date.getMonth();
      if (i % 7 == 0) {
        if (currentMonth != lastMonth && date.getDate() < 16) {
          cells.push(monthFormater.format(date));
        } else {
          cells.push("");
        }
      }

      const commitCount: CommitCount | null = data[i] || null;
      const cellLabel: string = `${commitCount || "No"} commits on ${dateFormater.format(date)}`;
      cells.push({ cellLabel, commitCount });

      lastMonth = currentMonth;
      date.setDate(date.getDate() + 1);
    }

    return cells;
  }, [data, startDate]);

  return (
    <div className="mb-4 flex flex-col items-start">
      <h2 className="font-bold text-lg mb-4">Preview</h2>
      <div
        className="grid grid-rows-8 grid-flow-col gap-2"
        aria-label="commit-graph"
      >
        <div></div>
        <div></div>
        <div className="mr-2 font-bold">Mon</div>
        <div></div>
        <div className="mr-2 font-bold">Wed</div>
        <div></div>
        <div className="mr-2 font-bold">Fri</div>
        <div></div>
        {cells.map((cell, idx) => {
          if (isString(cell)) {
            console.log(cell);
            return (
              <div key={idx} className="w-5 h-5 font-bold whitespace-nowrap">
                {cell}
              </div>
            );
          }

          const isFilled = cell.commitCount !== null;
          const colorClass = getColorForCommit(cell.commitCount);
          const borderClass = isFilled ? "border border-black/0" : "border";
          return (
            <div
              key={idx}
              className={`w-5 h-5 rounded-sm tooltip tooltip-info ${colorClass} ${borderClass} transition-colors duration-150`}
              // title={cell.cellLabel}
              data-tip={cell.cellLabel}
            />
          );
        })}
      </div>
    </div>
  );
}

export default Preview;
