use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use csv;
use serde::{Deserialize, Serialize};
use zip::{result::ZipError, ZipArchive};

use deserialize::{
    comma_separated, hhmm_time, line_separated, mm_dd_yy_date, non_null_bool, FromCsv,
};

use std::{
    collections::HashMap,
    convert::From,
    fmt,
    io::{Read, Seek},
};

mod time_period;

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
}

pub fn sort_map<K, V>(map: HashMap<K, V>) -> Vec<(K, V)>
where
    K: Ord,
{
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| (a.0).cmp(&b.0));
    v
}

#[derive(Serialize, Deserialize)]
pub struct AdverseEventRecord {
    #[serde(rename = "Date", deserialize_with = "mm_dd_yy_date::deserialize")]
    pub date: NaiveDate,
    #[serde(rename = "MRN")]
    pub mrn: String,
    #[serde(rename = "Episode ID")]
    pub episode_id: String,
    #[serde(rename = "Patient Name")]
    pub patient_name: String,
    #[serde(rename = "Diagnosis")]
    pub diagnosis: String,
    #[serde(rename = "Procedure")]
    pub procedure: String,
    #[serde(rename = "Anesthesiologist")]
    pub anesthesiologist: String,
    #[serde(
        rename = "Anesthesia Staff",
        deserialize_with = "line_separated::deserialize"
    )]
    pub anesthesia_staff: Vec<String>,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(
        rename = "Adverse Events",
        deserialize_with = "comma_separated::deserialize"
    )]
    pub adverse_events: Vec<String>,
    #[serde(rename = "ASA")]
    pub asa: u8,

    #[serde(rename = "An Start", deserialize_with = "hhmm_time::deserialize")]
    pub an_start: NaiveTime,
    #[serde(rename = "An Stop", deserialize_with = "hhmm_time::deserialize")]
    pub an_stop: NaiveTime,

    #[serde(rename = "Smoker?", deserialize_with = "non_null_bool::deserialize")]
    pub smoker: bool,
    #[serde(rename = "Age (Years)")]
    pub age: u8,
    #[serde(rename = "BMI")]
    pub bmi: f64,
}

impl FromCsv for AdverseEventRecord {}

#[derive(Debug)]
pub enum Error {
    DecompressError(ZipError),
    CsvError(csv::Error),
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
