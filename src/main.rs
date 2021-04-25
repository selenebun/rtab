use std::error::Error;
use std::fmt::Write;
use std::process;
use std::result;

use clap::{crate_authors, crate_version, App, Arg};
use csv::{ReaderBuilder, StringRecord, Trim};

type Result<T> = result::Result<T, Box<dyn Error>>;

/// A struct containing all the necessary table data.
pub struct Table {
    records: Vec<StringRecord>,
    widths: Vec<usize>,
}

impl Table {
    /// Create a table from a path.
    pub fn from_path(path: &str) -> Result<Self> {
        let records = Self::parse_records(path)?;
        let widths = Self::calculate_widths(&records);

        Ok(Self { records, widths })
    }

    /// Format a basic table.
    pub fn basic_format(&self) -> Result<String> {
        let mut output = String::new();
        for record in &self.records {
            for (i, field) in record.iter().enumerate() {
                write!(output, "{:width$}", field, width = self.widths[i] + 1)?;
            }

            // Trim trailing whitespace.
            let len = output.rfind(|c| !char::is_whitespace(c)).unwrap_or(0) + 1;
            output.truncate(len);

            writeln!(output)?;
        }

        Ok(output)
    }

    /// Format a fancy table.
    pub fn fancy_format(&self, headers: bool, separators: bool) -> Result<String> {
        let mut output = String::new();

        // Initial separator.
        for (i, width) in self.widths.iter().enumerate() {
            let vertical = match i {
                0 => "┌",
                _ => "┬",
            };
            write!(output, "{}{:─<width$}", vertical, "", width = width + 2)?;
        }
        writeln!(output, "┐")?;

        for (i, record) in self.records.iter().enumerate() {
            // Separator.
            if (separators && i > 0) || (headers && i == 1) {
                for (j, width) in self.widths.iter().enumerate() {
                    let vertical = match j {
                        0 => "├",
                        _ => "┼",
                    };
                    write!(output, "{}{:─<width$}", vertical, "", width = width + 2)?;
                }
                writeln!(output, "┤")?;
            }

            // Table data.
            for (j, field) in record.iter().enumerate() {
                write!(output, "│ {:width$} ", field, width = self.widths[j])?;
            }
            writeln!(output, "│")?;
        }

        // Final separator.
        for (i, width) in self.widths.iter().enumerate() {
            let vertical = match i {
                0 => "└",
                _ => "┴",
            };
            write!(output, "{}{:─<width$}", vertical, "", width = width + 2)?;
        }
        writeln!(output, "┘")?;

        Ok(output)
    }

    /// Calculate widths of each record.
    fn calculate_widths(records: &[StringRecord]) -> Vec<usize> {
        // Find the maximum width per column.
        let len = records.first().map_or(0, |r| r.len());
        records.iter().fold(vec![0; len], |acc, r| {
            acc.iter()
                .zip(r.iter())
                .map(|e| (*e.0).max(e.1.len()))
                .collect()
        })
    }

    /// Read records from a file.
    fn parse_records(path: &str) -> csv::Result<Vec<StringRecord>> {
        ReaderBuilder::new()
            .has_headers(false)
            .trim(Trim::All)
            .from_path(path)?
            .records()
            .collect()
    }
}

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
        .arg(
            Arg::with_name("headers")
                .long("headers")
                .short("h")
                .help("Use separators for first row in fancy tables"),
        )
        .arg(
            Arg::with_name("separators")
                .long("separators")
                .short("s")
                .help("Use no separators for fancy tables"),
        )
        .get_matches();

    // Parse table from CSV data.
    let path = matches.value_of("FILE").unwrap();
    let table = Table::from_path(path).unwrap_or_else(|e| {
        eprintln!("Error parsing file: {}", e);
        process::exit(1);
    });

    // Generate formatted table.
    let style = matches.value_of("STYLE").unwrap_or("basic");
    let headers = matches.is_present("headers");
    let separators = matches.is_present("separators");
    let output = match style {
        "basic" => table.basic_format(),
        "fancy" => table.fancy_format(headers, separators),
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
