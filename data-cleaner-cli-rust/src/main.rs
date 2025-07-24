use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

/// A CLI tool to clean a CSV file by removing rows with missing fields
#[derive(Parser, Debug)]
#[command(name = "Data Cleaner")]
#[command(version = "1.0")]
#[command(about = "Clean CSVs by removing invalid rows", long_about = None)]
struct Args {
    /// Input CSV file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output CSV file path
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    name: String,
    age: u32,
    email: String,
}

fn run(input: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::open(input)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let out_file = File::create(output)?;
    let mut wtr = WriterBuilder::new().from_writer(out_file);

    let mut cleaned = 0;
    let mut skipped = 0;

    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => {
                wtr.serialize(record)?;
                cleaned += 1;
            }
            Err(_) => {
                skipped += 1;
            }
        }
    }

    wtr.flush()?;
    println!("✅ Cleaned rows: {}", cleaned);
    println!("⚠️ Skipped rows (invalid): {}", skipped);
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args.input, args.output) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}
