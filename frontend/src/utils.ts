import { CommitCount } from "./constants.ts";

function isString(s: unknown) {
  return typeof s === "string" || s instanceof String;
}

function debounce(callback: (...args: unknown[]) => unknown, wait: number) {
  let timeoutId: number | undefined = undefined;
  return (...args: unknown[]) => {
    window.clearTimeout(timeoutId);
    timeoutId = window.setTimeout(() => {
      callback(...args);
    }, wait);
  };
}

const NEXT_COMMIT_COUNT = new Map([
  [null, CommitCount.Zero],
  [CommitCount.Zero, CommitCount.Few],
  [CommitCount.Few, CommitCount.Some],
  [CommitCount.Some, CommitCount.Many],
  [CommitCount.Many, CommitCount.ALot],
  [CommitCount.ALot, CommitCount.Zero],
]);

function nextCommitCount(c: CommitCount | null): CommitCount {
  return NEXT_COMMIT_COUNT.get(c) as CommitCount;
}

function isoDate(d: Date) {
  return d.toISOString().split("T")[0];
}

export { isString, debounce, nextCommitCount, isoDate };
