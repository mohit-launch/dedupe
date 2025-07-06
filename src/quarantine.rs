use crate::report::ReportEntry;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn quarantine_dup(report: &[ReportEntry], quarantine_dir: &Path) -> Result<()> {
    fs::create_dir_all(quarantine_dir).with_context(|| {
        format!(
            "Failed to create quarantine directory: {}",
            quarantine_dir.display()
        )
    })?;

    for entry in report {
        if entry.files.len() < 2 {
            continue;
        }

        let _duplicate = &entry.files[0];
        let duplicates = &entry.files[1..];

        for duplicate in duplicates {
            let duplicate_path = PathBuf::from(duplicate);
            let file_name = duplicate_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file path: {}", duplicate))?;

            let dest_path = quarantine_dir.join(file_name);

            let mut count = 1;
            let mut final_dest = dest_path.clone();
            while final_dest.exists() {
                let new_name = format!(
                    "{}_{}{}",
                    final_dest.file_stem().unwrap().to_string_lossy(),
                    count,
                    final_dest
                        .extension()
                        .map(|ext| format!(".{}", ext.to_string_lossy()))
                        .unwrap_or_default()
                );
                final_dest = quarantine_dir.join(new_name);
                count += 1;
            }

            fs::rename(&duplicate_path, &final_dest).with_context(|| {
                format!(
                    "Failed to move {} to quarantine at {}",
                    duplicate_path.display(),
                    final_dest.display()
                )
            })?;
            println!("Quarantined: {} -> {}", duplicate, final_dest.display());
        }
    }
    println!("Quarantine Complete ✅");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use crate::report::ReportEntry;

    fn to_report_entry(paths: Vec<PathBuf>) -> ReportEntry {
        ReportEntry {
            hash: "Dummy hash".to_string(),
            files: paths.into_iter().map(|p| p.to_string_lossy().to_string()).collect(),
        }
    }
    #[test]
    fn test_quarantine_moves_duplicates() {
        let temp_src = tempdir().unwrap();
        let temp_dst = tempdir().unwrap();

        let file1 = temp_src.path().join("dup1.txt");
        let file2 = temp_src.path().join("dup2.txt");

        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Hello from file 1").unwrap();

        let mut f2 = File::create(&file2).unwrap();
        writeln!(f2, "Hello from file 2").unwrap();

        let report = vec![to_report_entry(vec![file1.clone(), file2.clone()])];

        let result = quarantine_dup(&report, temp_dst.path());
        assert!(result.is_ok());

        // File1 remains, file2 is moved
        assert!(file1.exists());
        let quarantined = temp_dst.path().join("dup2.txt");
        assert!(quarantined.exists());
    }

    #[test]
    fn test_quarantine_skips_entries_with_single_file() {
        let temp_src = tempdir().unwrap();
        let quarantine_dir = tempdir().unwrap();

        let file = temp_src.path().join("unique.txt");
        let mut f = File::create(&file).unwrap();
        writeln!(f, "I am unique").unwrap();

        let report = vec![to_report_entry(vec![file.clone()])];
        let result = quarantine_dup(&report, quarantine_dir.path());
        assert!(result.is_ok());

        assert!(file.exists());
        assert!(quarantine_dir.path().read_dir().unwrap().next().is_none());
    }

    #[test]
    fn test_quarantine_creates_directory() {
        let temp_src = tempdir().unwrap();
        let quarantine_dir = temp_src.path().join("quarantine");

        let file1 = temp_src.path().join("sample.txt");
        let file2 = temp_src.path().join("sample_dup.txt");

        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "original").unwrap();
        let mut f2 = File::create(&file2).unwrap();
        writeln!(f2, "duplicate").unwrap();

        let report = vec![to_report_entry(vec![file1.clone(), file2.clone()])];

        let result = quarantine_dup(&report, &quarantine_dir);
        assert!(result.is_ok());
        assert!(file1.exists());
        assert!(quarantine_dir.join("sample_dup.txt").exists());
    }
}
