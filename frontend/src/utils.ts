import { CommitCount } from "./constants.ts";

function debounce(callback: (...args: unknown[]) => unknown, wait: number) {
  if (wait === 0) {
    return callback;
  }

  let timeoutId: number | undefined = undefined;
  return (...args: unknown[]) => {
    window.clearTimeout(timeoutId);
    timeoutId = window.setTimeout(() => {
      callback(...args);
    }, wait);
  };
}

let NEXT_COMMIT_COUNT: Map<CommitCount | null, CommitCount> | null = null;

function nextCommitCount(c: CommitCount | null): CommitCount {
  if (NEXT_COMMIT_COUNT == null) {
    NEXT_COMMIT_COUNT = new Map([
      [null, CommitCount.Zero],
      [CommitCount.Zero, CommitCount.Few],
      [CommitCount.Few, CommitCount.Some],
      [CommitCount.Some, CommitCount.Many],
      [CommitCount.Many, CommitCount.ALot],
      [CommitCount.ALot, CommitCount.Zero],
    ]);
  }
  return NEXT_COMMIT_COUNT.get(c) as CommitCount;
}

function isoDate(d: Date) {
  return d.toISOString().split("T")[0];
}

function dayDiff(a: string, b: string) {
  const date1 = new Date(a);
  const date2 = new Date(b);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  const diffTime = Math.abs(date2 - date1);
  return Math.floor(diffTime / (1000 * 60 * 60 * 24));
}

const dateFormater = new Intl.DateTimeFormat("en-US", {
  month: "short",
  day: "numeric",
});

const monthFormater = new Intl.DateTimeFormat("en-US", { month: "short" });

export { debounce, nextCommitCount, isoDate, dayDiff, dateFormater, monthFormater };
