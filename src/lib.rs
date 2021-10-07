use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Weekday};
use csv;
#[cfg(feature = "gen-fake")]
use fake::{
    faker::{
        lorem::en::{Word, Words},
        name::en::Name,
    },
    Dummy, Fake,
};
use serde::{Deserialize, Serialize};
use zip::{result::ZipError, ZipArchive};

use deserialize::{
    comma_separated, hhmm_time, line_separated, mm_dd_yy_date, non_null_bool, nullable_yes_no_bool,
    FromCsv,
};

use std::{
    collections::HashMap,
    convert::From,
    fmt, hash,
    io::{self, Read, Seek},
};

mod breakdown;
mod time_period;

pub use breakdown::*;
pub use time_period::*;

pub struct AdverseEvents {
    pub records: Vec<AdverseEventRecord>,
}

impl AdverseEvents {
    pub fn new() -> Self {
        AdverseEvents {
            records: Vec::new(),
        }
    }

    pub fn from_zip<R>(data: R) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let mut archive = ZipArchive::new(data)?;
        let csv = archive.by_index(0)?;
        if csv.is_file() && csv.name().ends_with(".csv") {
            Self::from_csv_reader(csv)
        } else {
            Err(Error::DecompressError(ZipError::FileNotFound))
        }
    }

    pub fn from_csv_reader<R>(reader: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let records = AdverseEventRecord::from_csv_reader(reader)?;
        Ok(AdverseEvents { records })
    }

    pub fn view(&self) -> AdverseEventsView {
        self.into()
    }
}

#[derive(Serialize)]
pub struct AdverseEventsView<'a> {
    pub records: Vec<&'a AdverseEventRecord>,
}

impl<'a> From<&'a AdverseEvents> for AdverseEventsView<'a> {
    fn from(events: &'a AdverseEvents) -> Self {
        AdverseEventsView {
            records: events.records.iter().collect(),
        }
    }
}

impl<'a> AdverseEventsView<'a> {
    pub fn empty() -> Self {
        AdverseEventsView {
            records: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn date_range(&self) -> Option<(NaiveDate, NaiveDate)> {
        // Using iterators would be more ergonomic but I want to avoid two iterations
        if self.records.is_empty() {
            None
        } else {
            let mut min: NaiveDate = self.records[0].date;
            let mut max: NaiveDate = self.records[0].date;
            for record in self.records.iter().skip(1) {
                min = min.min(record.date);
                max = max.max(record.date);
            }

            Some((min, max))
        }
    }

    pub fn event_counts(&self) -> HashMap<&str, u32> {
        let mut counts: HashMap<&str, u32> = HashMap::new();

        for record in &self.records {
            for event in &record.adverse_events {
                *counts.entry(event).or_default() += 1;
            }
        }

        counts
    }

    pub fn with_any_event(&self) -> Self {
        self.with_filter(|record| !record.adverse_events.is_empty())
    }

    pub fn between(&self, start: NaiveDate, end: NaiveDate) -> Self {
        self.with_filter(|record| start <= record.date && record.date <= end)
    }

    pub fn between_times(&self, start: NaiveTime, end: NaiveTime) -> Self {
        self.with_filter(|record| record.an_stop > start && record.an_start < end)
    }

    pub fn with_event(&self, event: &str) -> Self {
        self.with_filter(|record| record.adverse_events.iter().any(|e| e == event))
    }

    pub fn by_anesthesiologist(&self, anesthesiologist_name: &str) -> Self {
        self.with_filter(|record| record.anesthesiologist == anesthesiologist_name)
    }

    pub fn with_staff(&self, staff_name: &str) -> Self {
        self.with_filter(|record| {
            record
                .anesthesia_staff
                .iter()
                .any(|name| name == staff_name)
        })
    }

    pub fn with_procedure(&self, proc_name: &str) -> Self {
        self.with_filter(|record| record.procedure == proc_name)
    }

    pub fn with_filter<F>(&self, filter: F) -> Self
    where
        F: FnMut(&&&AdverseEventRecord) -> bool,
    {
        Self {
            records: self.records.iter().filter(filter).copied().collect(),
        }
    }

    pub fn count<F>(&self, count_if: F) -> usize
    where
        F: FnMut(&&&AdverseEventRecord) -> bool,
    {
        self.records.iter().filter(count_if).count()
    }

    pub fn by_period<F>(&self, period: Period, filter: F) -> Vec<DatePeriodView<'a>>
    where
        F: FnMut(&&&AdverseEventRecord) -> bool,
    {
        match period {
            Period::Day => {
                let mut by_date =
                    group_by_owned(self.records.iter().filter(filter).copied(), |record| {
                        record.date
                    });

                let min: Option<NaiveDate> = by_date.keys().copied().min();
                let max: Option<NaiveDate> = by_date.keys().copied().max();

                if let (Some(min), Some(max)) = (min, max) {
                    let step = Duration::days(1);
                    let mut date = min;
                    let mut ret = Vec::new();

                    while date <= max {
                        let records = by_date.remove(&date).unwrap_or_default();
                        ret.push(DatePeriodView {
                            period,
                            start: date,
                            end: date,
                            value: AdverseEventsView { records },
                        });
                        date += step;
                    }

                    ret
                } else {
                    Vec::new()
                }
            }
            Period::Week => {
                let mut by_week = group_by_owned(self.records.iter().copied(), |record| {
                    (record.date.year(), record.date.iso_week().week())
                });

                let dates: Vec<_> = by_week
                    .keys()
                    .map(|(year, week)| NaiveDate::from_isoywd(*year, *week, Weekday::Mon))
                    .collect();
                let min: Option<&NaiveDate> = dates.iter().min();
                let max: Option<&NaiveDate> = dates.iter().max();

                if let (Some(min), Some(max)) = (min, max) {
                    let mut date = *min;

                    let mut ret = Vec::new();

                    while date <= *max {
                        let records = by_week
                            .remove(&(date.year(), date.iso_week().week()))
                            .unwrap_or_default();

                        ret.push(DatePeriodView {
                            period,
                            start: date,
                            end: NaiveDate::from_isoywd(
                                date.year(),
                                date.iso_week().week(),
                                Weekday::Sun,
                            ),
                            value: AdverseEventsView { records },
                        });

                        date += Duration::weeks(1);
                    }

                    ret
                } else {
                    Vec::new()
                }
            }
            Period::Month => {
                let mut by_month = group_by_owned(self.records.iter().copied(), |record| {
                    (record.date.year(), record.date.month())
                });
                let dates: Vec<_> = by_month
                    .keys()
                    .map(|(year, month)| NaiveDate::from_ymd(*year, *month, 1))
                    .collect();
                let min: Option<&NaiveDate> = dates.iter().min();
                let max: Option<&NaiveDate> = dates.iter().max();

                if let (Some(min), Some(max)) = (min, max) {
                    let mut date = *min;

                    let mut ret = Vec::new();

                    while date <= *max {
                        let records = by_month
                            .remove(&(date.year(), date.month()))
                            .unwrap_or_default();

                        let next_date = if date.month() == 12 {
                            NaiveDate::from_ymd(date.year(), date.month() + 1, 1)
                        } else {
                            NaiveDate::from_ymd(date.year() + 1, 1, 1)
                        };

                        ret.push(DatePeriodView {
                            period,
                            start: date,
                            end: next_date - Duration::days(1),
                            value: AdverseEventsView { records },
                        });
                        date = next_date;
                    }

                    ret
                } else {
                    Vec::new()
                }
            }
            Period::Year => {
                let mut by_year =
                    group_by_owned(self.records.iter().copied(), |record| record.date.year());

                let min: Option<i32> = by_year.keys().copied().min();
                let max: Option<i32> = by_year.keys().copied().max();

                if let (Some(min), Some(max)) = (min, max) {
                    let mut ret = Vec::new();

                    let mut year = min;

                    while year <= max {
                        let records = by_year.remove(&year).unwrap_or_default();
                        ret.push(DatePeriodView {
                            period,
                            start: NaiveDate::from_ymd(year, 1, 1),
                            end: NaiveDate::from_ymd(year, 12, 31),
                            value: AdverseEventsView { records },
                        });

                        year += 1;
                    }

                    ret
                } else {
                    Vec::new()
                }
            }
        }
    }

    pub fn get_breakdown(&self, breakdown_type: BreakdownType) -> Vec<LabeledCount> {
        match breakdown_type {
            BreakdownType::WithComplications => {
                vec![
                    LabeledCount {
                        label: "With complications".to_string(),
                        value: self.count(|record| record.complications == Some(true)),
                    },
                    LabeledCount {
                        label: "Without complications".to_string(),
                        value: self.count(|record| record.complications == Some(false)),
                    },
                    LabeledCount {
                        label: "Unspecified complications".to_string(),
                        value: self.count(|record| record.complications.is_none()),
                    },
                ]
            }
            BreakdownType::WithEvent => {
                vec![
                    LabeledCount {
                        label: "With event".to_string(),
                        value: self.count(|record| !record.adverse_events.is_empty()),
                    },
                    LabeledCount {
                        label: "Without event".to_string(),
                        value: self.count(|record| record.adverse_events.is_empty()),
                    },
                ]
            }
            BreakdownType::PatientAge => {
                let range_size = 10;
                sort_map(group_by_owned(
                    self.records.iter().filter(|r| !r.adverse_events.is_empty()),
                    |record| record.age - (record.age % range_size),
                ))
                .into_iter()
                .map(|(range_start, records)| LabeledCount {
                    label: format!("{} - {}", range_start, range_start + range_size - 1),
                    value: records.len(),
                })
                .collect()
            }
            BreakdownType::PatientBmi => {
                let range_size = 5;
                sort_map(group_by_owned(
                    self.records.iter().filter(|r| !r.adverse_events.is_empty()),
                    |record| {
                        let bmi = record.bmi as usize;
                        bmi - (bmi % range_size)
                    },
                ))
                .into_iter()
                .map(|(range_start, records)| LabeledCount {
                    label: format!("{} - {}", range_start, range_start + range_size - 1),
                    value: records.len(),
                })
                .collect()
            }
            BreakdownType::PatientSmoker => {
                vec![
                    LabeledCount {
                        label: "Smoker".to_string(),
                        value: self
                            .count(|record| !record.adverse_events.is_empty() && record.smoker),
                    },
                    LabeledCount {
                        label: "Non-smoker".to_string(),
                        value: self
                            .count(|record| !record.adverse_events.is_empty() && !record.smoker),
                    },
                ]
            }
        }
    }
}

pub fn sort_map<K, V>(map: HashMap<K, V>) -> Vec<(K, V)>
where
    K: Ord,
{
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| (a.0).cmp(&b.0));
    v
}

pub fn group_by<'a, F, K, T>(
    items: impl IntoIterator<Item = &'a T>,
    grouper: F,
) -> HashMap<&'a K, Vec<&'a T>>
where
    F: Fn(&T) -> &K,
    K: Eq + hash::Hash,
{
    let mut map: HashMap<&K, Vec<&T>> = HashMap::new();

    for item in items {
        map.entry(grouper(item)).or_default().push(item)
    }

    map
}

pub fn group_by_owned<'a, F, K, T>(
    items: impl IntoIterator<Item = &'a T>,
    grouper: F,
) -> HashMap<K, Vec<&'a T>>
where
    F: Fn(&T) -> K,
    K: Eq + hash::Hash,
{
    let mut map: HashMap<K, Vec<&T>> = HashMap::new();

    for item in items {
        map.entry(grouper(item)).or_default().push(item)
    }

    map
}

#[cfg(feature = "gen-fake")]
mod gen_fake {
    use super::*;
    use rand::Rng;

    pub(super) struct DateWithinYears(pub i32);

    impl Dummy<DateWithinYears> for NaiveDate {
        fn dummy_with_rng<R: Rng + ?Sized>(rd: &DateWithinYears, rng: &mut R) -> NaiveDate {
            let year = rng.gen_range((2020 - rd.0)..=2020);
            let month = rng.gen_range(1..=12);
            let day = rng.gen_range(
                1..=match month {
                    2 => 28,
                    9 | 4 | 6 | 11 => 30,
                    _ => 31,
                },
            );

            NaiveDate::from_ymd(year, month, day)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "gen-fake", derive(Dummy))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AdverseEventRecord {
    #[serde(rename(deserialize = "Date"), with = "mm_dd_yy_date")]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "gen_fake::DateWithinYears(2)"))]
    pub date: NaiveDate,
    #[serde(rename(deserialize = "MRN"))]
    pub mrn: String,
    #[serde(rename(deserialize = "Episode ID"))]
    pub episode_id: String,
    #[serde(rename(deserialize = "Patient Name"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "Name()"))]
    pub patient_name: String,
    #[serde(rename(deserialize = "Diagnosis"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "Word()"))]
    pub diagnosis: String,
    #[serde(rename(deserialize = "Procedure"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "Word()"))]
    pub procedure: String,
    #[serde(rename(deserialize = "Anesthesiologist"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "Name()"))]
    pub anesthesiologist: String,
    #[serde(rename(deserialize = "Anesthesia Staff"), with = "line_separated")]
    pub anesthesia_staff: Vec<String>,
    #[serde(rename(deserialize = "Location"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "Word()"))]
    pub location: String,
    #[serde(
        rename(deserialize = "Anesthesia Complications"),
        with = "nullable_yes_no_bool"
    )]
    pub complications: Option<bool>,
    #[serde(rename(deserialize = "Adverse Events"), with = "comma_separated")]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "Words(0..5)"))]
    pub adverse_events: Vec<String>,
    #[serde(rename(deserialize = "ASA"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "0..=5"))]
    pub asa: u8,

    #[serde(rename(deserialize = "An Start"), with = "hhmm_time")]
    pub an_start: NaiveTime,
    #[serde(rename(deserialize = "An Stop"), with = "hhmm_time")]
    pub an_stop: NaiveTime,

    #[serde(rename(deserialize = "Smoker?"), with = "non_null_bool")]
    pub smoker: bool,
    #[serde(rename(deserialize = "Age (Years)"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "0..120"))]
    pub age: u8,
    #[serde(rename(deserialize = "BMI"))]
    #[cfg_attr(feature = "gen-fake", dummy(faker = "10.0..40.0"))]
    pub bmi: f64,
}

impl FromCsv for AdverseEventRecord {}

#[derive(Debug)]
pub enum Error {
    DecompressError(ZipError),
    CsvError(csv::Error),
    IoError(io::Error),
    ParseError {
        type_name: &'static str,
        received: String,
    },
}

impl From<ZipError> for Error {
    fn from(e: ZipError) -> Self {
        Error::DecompressError(e)
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::CsvError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError {
                type_name,
                received,
            } => {
                write!(f, "ParseError: invalid {}: {}", type_name, received)
            }
            err => write!(f, "{:?}", err),
        }
    }
}

impl std::error::Error for Error {}
