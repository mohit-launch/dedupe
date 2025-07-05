mod hashing;
mod report;
mod scanning;
mod processor;
mod quarantine;

use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use anyhow::Result;
use std::collections::HashMap;

use crate::hashing::HashAlgorithm;
use crate::report::ReportEntry;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to scan for duplicates
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,

    /// Output report file
    #[arg(short, long, default_value = "duplicates.json")]
    output: String,

    /// Hash algorithm to use (sha256, blake3, xxhash)
    #[arg(short, long, default_value = "blake3")]
    algorithm: String,

    /// Quarantine duplicates (keeps one original)
    #[arg(short, long)]
    quarantine: bool,

    /// Quarantine directory path
    #[arg(long, default_value = "quarantine")]
    quarantine_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Parse hash algorithm
    let algorithm = match args.algorithm.to_lowercase().as_str() {
        "sha256" => HashAlgorithm::SHA256,
        "blake3" => HashAlgorithm::Blake3,
        "xxhash" => HashAlgorithm::XxHash,
        _ => {
            eprintln!("Invalid algorithm specified. Using Blake3 as default.");
            HashAlgorithm::Blake3
        }
    };

    println!("Scanning directory: {}", args.directory.display());
    let scan_start = Instant::now();
    let files = scanning::scan_directory_parallel(&args.directory);
    println!("Found {} files in {:?}", files.len(), scan_start.elapsed());

    if files.is_empty() {
        println!("No files found to process.");
        return Ok(());
    }

    println!("Computing hashes using {:?}...", algorithm);
    let hash_start = Instant::now();
    let results = processor::process_files_parallel(files, algorithm)?;
    println!("Hashing completed in {:?}", hash_start.elapsed());

    println!("Generating duplicate report...");
    report::generate_report(&results, &args.output);
    println!("Report saved to {}", args.output);

    let mut map:HashMap<String, Vec<String>> = HashMap::new();
    for (path, hash) in &results {
        map.entry(hash.clone())
            .or_default()
            .push(path.display().to_string());
    }

    let dedup_report: Vec<ReportEntry> = map
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(hash, files)| ReportEntry { hash, files })
        .collect();

    if args.quarantine {
        // Quarantine duplicates if requested
        quarantine::quarantine_dup(&dedup_report, &args.quarantine_dir)?;
    }

    // Optionally, you can serialize dedup_report separately if needed,
    // but do not call generate_report with the wrong type.

    Ok(())
}