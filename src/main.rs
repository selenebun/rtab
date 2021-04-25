use std::error::Error;
use std::fmt::Write;
use std::process;
use std::result;

use clap::{crate_authors, crate_version, App, Arg};
use csv::{ReaderBuilder, StringRecord, Trim};

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() {
    let matches = App::new("rtab")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Generate tables from CSV.")
        .arg(Arg::with_name("FILE").required(true))
        .arg(
            Arg::with_name("STYLE")
                .long("style")
                .help("Sets table style")
                .takes_value(true)
                .possible_values(&["basic", "fancy"]),
        )
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

    // Generate formatted table.
    let style = matches.value_of("STYLE").unwrap_or("basic");
    let widths = calculate_widths(&records);
    let output = match style {
        "basic" => basic_table(&records, &widths),
        "fancy" => fancy_table(&records, &widths),
        _ => unreachable!(),
    };

    // Print table.
    match output {
        Ok(output) => print!("{}", output),
        Err(e) => {
            eprintln!("Error formatting output: {}", e);
            process::exit(1);
        }
    }
}

/// Generate a basic table.
fn basic_table(records: &[StringRecord], widths: &[usize]) -> Result<String> {
    // Build output string.
    let mut output = String::new();
    for record in records {
        for (i, field) in record.iter().enumerate() {
            write!(output, "{:width$}", field, width = widths[i] + 1)?;
        }

        // Trim trailing whitespace.
        let len = output.rfind(|c| !char::is_whitespace(c)).unwrap_or(0) + 1;
        output.truncate(len);

        writeln!(output)?;
    }

    Ok(output)
}

/// Calculate widths of each record.
fn calculate_widths(records: &[StringRecord]) -> Vec<usize> {
    // Find the maximum width per column.
    let length = records.first().map_or(0, |r| r.len());
    records.iter().fold(vec![0; length], |acc, r| {
        acc.iter()
            .zip(r.iter())
            .map(|e| (*e.0).max(e.1.len()))
            .collect()
    })
}

/// Generate a fancy table.
fn fancy_table(records: &[StringRecord], widths: &[usize]) -> Result<String> {
    // Build output string.
    let mut output = String::new();
    for (i, record) in records.iter().enumerate() {
        // Determine correct character set.
        let (beginning, middle, end) = match i {
            0 => ("┌", "┬", "┐"),
            _ => ("├", "┼", "┤"),
        };

        // Separator.
        for (j, width) in widths.iter().enumerate() {
            let vertical = match j {
                0 => beginning,
                _ => middle,
            };
            write!(output, "{}{:─<width$}", vertical, "", width = width + 2)?;
        }
        writeln!(output, "{}", end)?;

        // Table data.
        for (j, field) in record.iter().enumerate() {
            write!(output, "│ {:width$} ", field, width = widths[j])?;
        }
        writeln!(output, "│")?;
    }

    // Final separator.
    for (i, width) in widths.iter().enumerate() {
        let vertical = match i {
            0 => "└",
            _ => "┴",
        };
        write!(output, "{}{:─<width$}", vertical, "", width = width + 2)?;
    }
    writeln!(output, "┘")?;

    Ok(output)
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
