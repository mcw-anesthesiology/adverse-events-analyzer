use chrono::{NaiveDate, NaiveTime};
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

pub struct AdverseEvents {
    pub records: Vec<AdverseEventRecord>,
}

impl AdverseEvents {
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
    pub fn event_counts(&self) -> HashMap<&str, u32> {
        let mut counts: HashMap<&str, u32> = HashMap::new();

        for record in &self.records {
            for event in &record.adverse_events {
                *counts.entry(event).or_default() += 1;
            }
        }

        counts
    }

    pub fn between(self, start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            records: self
                .records
                .into_iter()
                .filter(|record| start <= record.date && record.date <= end)
                .collect(),
        }
    }

    pub fn with_event(self, event: &str) -> Self {
        Self {
            records: self
                .records
                .into_iter()
                .filter(|record| record.adverse_events.iter().any(|e| e == event))
                .collect(),
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

#[derive(Serialize, Deserialize)]
pub struct AdverseEventRecord {
    #[serde(rename = "Date", deserialize_with = "mm_dd_yy_date::deserialize")]
    date: NaiveDate,
    #[serde(rename = "MRN")]
    mrn: String,
    #[serde(rename = "Episode ID")]
    episode_id: String,
    #[serde(rename = "Patient Name")]
    patient_name: String,
    #[serde(rename = "Diagnosis")]
    diagnosis: String,
    #[serde(rename = "Procedure")]
    procedure: String,
    #[serde(rename = "Anesthesiologist")]
    anesthesiologist: String,
    #[serde(
        rename = "Anesthesia Staff",
        deserialize_with = "line_separated::deserialize"
    )]
    anesthesia_staff: Vec<String>,
    #[serde(rename = "Location")]
    location: String,
    #[serde(
        rename = "Adverse Events",
        deserialize_with = "comma_separated::deserialize"
    )]
    adverse_events: Vec<String>,
    #[serde(rename = "ASA")]
    asa: u8,

    #[serde(rename = "An Start", deserialize_with = "hhmm_time::deserialize")]
    an_start: NaiveTime,
    #[serde(rename = "An Stop", deserialize_with = "hhmm_time::deserialize")]
    an_stop: NaiveTime,

    #[serde(rename = "Smoker?", deserialize_with = "non_null_bool::deserialize")]
    smoker: bool,
    #[serde(rename = "Age (Years)")]
    age: u8,
    #[serde(rename = "BMI")]
    bmi: f64,
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
