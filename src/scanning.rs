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
#[cfg(test)]
mod tests{
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_scan_directory_parallel(){
        let dir=tempdir().unwrap();
        let file_path1=dir.path().join("file1.txt");
        let file_path2=dir.path().join("file2.txt");

        let mut file1=File::create(&file_path1).unwrap();
        writeln!(file1,"This is file 1").unwrap();

        let mut file2=File::create(&file_path2).unwrap();
        writeln!(file2,"This is file2").unwrap();

        let res=scan_directory_parallel(dir.path());
        assert_eq!(res.contains(&file_path1),true);
        assert_eq!(res.contains(&file_path2),true);
    }
     #[test]
    fn test_scan_directory_empty_folder() {
        let dir = tempdir().unwrap();

        let results = scan_directory_parallel(dir.path());

        assert!(results.is_empty(), "Expected empty vector for empty directory");
    }

    #[test]
    fn test_scan_directory_ignores_directories() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file_path = subdir.join("nested.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Nested file").unwrap();

        let results = scan_directory_parallel(dir.path());

        // Assuming recursive scan
        assert!(results.contains(&file_path), "Expected recursive scan to include nested file");
    }
}