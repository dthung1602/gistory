use std::ops::Add;

use chrono::TimeDelta;

pub struct DateRangeIter<DateType>
where
    DateType: Add<TimeDelta, Output = DateType> + PartialOrd + Copy,
{
    current: DateType,
    count: usize,
}

impl<DateType> DateRangeIter<DateType>
where
    DateType: Add<TimeDelta, Output = DateType> + PartialOrd + Copy,
{
    pub fn new(start: DateType, days: usize) -> Self {
        Self {
            current: start,
            count: days,
        }
    }
}

impl<DateType> Iterator for DateRangeIter<DateType>
where
    DateType: Add<TimeDelta, Output = DateType> + PartialOrd + Copy,
{
    type Item = DateType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        let current = self.current;
        self.current = current.add(TimeDelta::days(1));
        Some(current)
    }
}
