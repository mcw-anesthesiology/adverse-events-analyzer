use clap::{App, Arg, SubCommand};
use csv;

use adverse_events::{sort_map, AdverseEvents, AdverseEventsView};

use std::{ffi::OsStr, fs::File, io, path::Path};

fn main() {
    let matches = App::new("Adverse events analyzer")
        .arg(
            Arg::with_name("input")
                .help("Input record (either CSV, or ZIP archive with a single CSV inside)")
                .required(true),
        )
        .subcommand(SubCommand::with_name("counts"))
        .get_matches();

    let path = Path::new(matches.value_of("input").unwrap());
    let file = File::open(path).unwrap();

    let adverse_events = match path.extension().and_then(OsStr::to_str) {
        Some("zip") => AdverseEvents::from_zip(file),
        Some("csv") => AdverseEvents::from_csv_reader(file),
        Some(ext) => panic!("unsupported extension: {}", ext),
        None => panic!("input path missing extension"),
    }
    .unwrap();

    let view: AdverseEventsView = adverse_events.view();

    match matches.subcommand() {
        ("counts", _) => {
            event_counts(&view).unwrap();
        }
        (command, _) => {
            panic!("unknown subcommand {}", command);
        }
    }
}

fn event_counts(view: &AdverseEventsView<'_>) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_writer(io::stdout());

    writer.write_record(&["Adverse event", "Count"])?;

    let mut total: u64 = 0;
    for (event, count) in sort_map(view.event_counts()) {
        total += count as u64;
        writer.write_record(&[event, &count.to_string()])?;
    }

    writer.write_record(&["Total", &total.to_string()])?;

    Ok(())
}
