use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn scan_directory_parallel(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    scan_parallel(dir, &mut files);
    files
}

fn scan_parallel(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        let entries: Vec<_> = entries.flatten().collect();

        let (dirs, current_files): (Vec<_>, Vec<_>) = entries
            .par_iter()
            .partition(|entry| entry.path().is_dir());

        let sub_files: Vec<_> = current_files
            .into_par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                if path.is_file() { Some(path) } else { None }
            })
            .collect();

        files.extend(sub_files);

        let nested_files: Vec<Vec<PathBuf>> = dirs
            .into_par_iter()
            .map(|entry| {
                let mut sub_files = Vec::new();
                scan_parallel(&entry.path(), &mut sub_files);
                sub_files
            })
            .collect();

        for sublist in nested_files {
            files.extend(sublist);
        }
    }
}
