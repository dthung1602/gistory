import { isoDate } from "./utils.ts";

enum CommitCount {
  Zero = "Zero",
  Few = "Few",
  Some = "Some",
  Many = "Many",
  ALot = "ALot",
}

enum Font {
  SubwayTracker = "SubwayTracker",
}

const SUNDAY = 0;

const TODAY = new Date();
const NEXT_6_MONTH = new Date();
NEXT_6_MONTH.setMonth(TODAY.getMonth() + 6);
const TODAY_STR = isoDate(TODAY);
const NEXT_6_MONTH_STR = isoDate(NEXT_6_MONTH);

export { CommitCount, Font, SUNDAY, TODAY_STR, NEXT_6_MONTH_STR };
