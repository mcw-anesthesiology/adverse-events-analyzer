use super::*;

pub struct TimePeriodContainer<T> {
    start: NaiveDateTime,
    end: NaiveDateTime,
    value: T,
}

pub type TimePeriodView<'a> = TimePeriodContainer<AdverseEventsView<'a>>;

pub type TimePeriodCount = TimePeriodContainer<usize>;

impl<'a> TimePeriodView<'a> {
    pub fn to_count(&self) -> TimePeriodCount {
        TimePeriodCount {
            start: self.start,
            end: self.end,
            value: self.value.len(),
        }
    }
}
