use std::process;

use clap::{crate_authors, crate_version, App, Arg};
use csv::{ReaderBuilder, StringRecord, Trim};

fn main() {
    let matches = App::new("rtab")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Generate tables from CSV.")
        .arg(Arg::with_name("FILE").required(true))
        .get_matches();

    // Open input file for reading.
    let path = matches.value_of("FILE").unwrap();
    let records = match parse_records(path) {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    };
}

/// Read records from file.
fn parse_records(path: &str) -> csv::Result<Vec<StringRecord>> {
    ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .from_path(path)?
        .records()
        .collect()
}
