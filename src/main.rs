use adverse_events::{AdverseEvents, AdverseEventsView};

use std::{env, fs::File};

fn main() {
    let path = env::args()
        .skip(1)
        .next()
        .expect("must pass ZIP path as first argument");

    let file = File::open(path).unwrap();

    let adverse_events = AdverseEvents::from_zip(file).unwrap();

    let view: AdverseEventsView = adverse_events.view();

    for (event, count) in view.event_counts() {
        println!("{}: {}", event, count);
    }
}
