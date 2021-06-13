use chrono::{NaiveDate, NaiveTime};
use csv;
use serde::{Deserialize, Serialize};
use zip::{result::ZipError, ZipArchive};

use deserialize::{hhmm_time, line_separated, mm_dd_yy_date, non_null_bool, FromCsv};

use std::{
    convert::From,
    fmt,
    io::{Read, Seek},
};

pub struct AdverseEvents {
    pub records: Vec<AdverseEventRecord>,
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
        deserialize_with = "line_separated::deserialize"
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
}

#[derive(Debug)]
pub enum Error {
    DecompressError(ZipError),
    ParseError(csv::Error),
}

impl From<ZipError> for Error {
    fn from(e: ZipError) -> Self {
        Error::DecompressError(e)
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::ParseError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
