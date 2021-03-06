use super::*;

use std::str::FromStr;

#[derive(Serialize)]
pub struct DatePeriodContainer<T: Serialize> {
    pub period: Period,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub value: T,
}

pub type DatePeriodView<'a> = DatePeriodContainer<AdverseEventsView<'a>>;

pub type DatePeriodCount = DatePeriodContainer<usize>;
pub type DatePeriodPercentage = DatePeriodContainer<f64>;

impl<'a> DatePeriodView<'a> {
    pub fn to_count(&self) -> DatePeriodCount {
        DatePeriodCount {
            period: self.period,
            start: self.start,
            end: self.end,
            value: self.value.len(),
        }
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
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

pub enum TimeseriesType {
    EventCount,
    EventPercentage,
    ComplicationSpecifiedCount,
    ComplicationSpecifiedPercentage,
    ComplicationOccurredCount,
    ComplicationOccurredPercentage,
}

impl FromStr for TimeseriesType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "event" => Ok(TimeseriesType::EventCount),
            "eventPercentage" => Ok(TimeseriesType::EventPercentage),
            "complicationSpecified" => Ok(TimeseriesType::ComplicationSpecifiedCount),
            "complicationSpecifiedPercentage" => {
                Ok(TimeseriesType::ComplicationSpecifiedPercentage)
            }
            "complicationOccurred" => Ok(TimeseriesType::ComplicationOccurredCount),
            "complicationOccurredPercentage" => Ok(TimeseriesType::ComplicationOccurredPercentage),
            x => Err(crate::Error::ParseError {
                type_name: "TimeseriesType",
                received: x.to_string(),
            }),
        }
    }
}
