use super::*;

use std::str::FromStr;

#[derive(Serialize)]
pub struct DatePeriodContainer<T: Serialize> {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub value: T,
}

pub type DatePeriodView<'a> = DatePeriodContainer<AdverseEventsView<'a>>;

pub type DatePeriodCount = DatePeriodContainer<usize>;

impl<'a> DatePeriodView<'a> {
    pub fn to_count(&self) -> DatePeriodCount {
        DatePeriodCount {
            start: self.start,
            end: self.end,
            value: self.value.len(),
        }
    }
}

pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

impl FromStr for Period {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "day" => Ok(Period::Day),
            "week" => Ok(Period::Week),
            "month" => Ok(Period::Month),
            "year" => Ok(Period::Year),
            x => Err(crate::Error::ParseError {
                type_name: "Period",
                received: x.to_string(),
            }),
        }
    }
}
