use mcw_anesth_adverse_events_analyzer::AdverseEvents;

use std::{env, fs::File};

fn main() {
    let path = env::args()
        .skip(1)
        .next()
        .expect("must pass ZIP path as first argument");

    let file = File::open(path).unwrap();

    let adverse_events = AdverseEvents::from_zip(file).unwrap();

    dbg!(adverse_events.records.len());
}
