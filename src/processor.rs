use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use crate::hashing::{compute_hash, HashAlgorithm};
use anyhow::Context;

pub fn process_files_parallel(
    files: Vec<PathBuf>,
    algo: HashAlgorithm,
) -> anyhow::Result<Vec<(PathBuf, String)>> {
    // Early return if no files to process
    if files.is_empty() {
        return Ok(Vec::new());
    }

    let bar = ProgressBar::new(files.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/yellow}] {pos}/{len} ({eta})"
        )
        .context("Failed to set progress bar style")?
        .progress_chars("##-"),
    );

    let results: Vec<(PathBuf, String)> = files
        .into_par_iter()
        .filter_map(|path| {
            let result = compute_hash(&path, algo.clone())
                .map_err(|e| {
                    eprintln!("Error processing {}: {}", path.display(), e);
                    e
                })
                .ok();
            bar.inc(1);
            result.map(|hash| (path, hash))
        })
        .collect();

    bar.finish_with_message("Processing complete");
    Ok(results)
}