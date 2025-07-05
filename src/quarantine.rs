use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use crate::report::ReportEntry; 

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