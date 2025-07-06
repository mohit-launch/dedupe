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
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.yellow/green}] {pos}/{len} ({eta})"
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_process_files_parallel() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");

        let mut f1 = File::create(&file1).unwrap();
        f1.write_all(b"Hello, World!").unwrap();

        let mut f2 = File::create(&file2).unwrap();
        f2.write_all(b"Hello, World!").unwrap();

        let files = vec![file1, file2];
        let results = process_files_parallel(files, HashAlgorithm::Blake3).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, results[1].1); // Both files should have the same hash
    }
    #[test]
    fn test_process_files_parallel_empty() {
        let _dir = tempdir().unwrap();
        let files = vec![];
        let results = process_files_parallel(files, HashAlgorithm::Blake3).unwrap();
        assert!(results.is_empty());
    }
}