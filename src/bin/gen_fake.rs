use csv;
#[cfg(feature = "gen-fake")]
use fake::vec;

use adverse_events::AdverseEventRecord;

use std::{env, io};

fn main() {
    let num_records: usize = env::args()
        .nth(1)
        .map(|num| num.parse().expect("invalid number of records to generate"))
        .unwrap_or(10_000);

    let records: Vec<AdverseEventRecord> = vec![AdverseEventRecord; num_records];

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    writer
        .write_record(&[
            "Date",
            "MRN",
            "Episode ID",
            "Patient Name",
            "Diagnosis",
            "Procedure",
            "Anesthesiologist",
            "Anesthesia Staff",
            "Location",
            "Anesthesia Complications",
            "Adverse Events",
            "ASA",
            "An Start",
            "An Stop",
            "Smoker?",
            "Age (Years)",
            "BMI",
        ])
        .unwrap();

    for record in records {
        writer.serialize(record).unwrap();
    }
}
