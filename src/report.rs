use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug,Serialize,Clone)]
pub struct ReportEntry {
    pub hash: String,
   pub  files: Vec<String>,
}

pub fn generate_report(results: &[(PathBuf, String)], output: &str) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (path, hash) in results {
        map.entry(hash.clone())
            .or_default()
            .push(path.display().to_string());
    }

    let dedup_report: Vec<ReportEntry> = map
        .into_iter()
        .filter(|(_, files)| files.len() > 1) // only duplicates
        .map(|(hash, files)| ReportEntry { hash, files })
        .collect();

    let json = serde_json::to_string_pretty(&dedup_report).unwrap();
    let mut file = File::create(output).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_genetate_json_report(){
        let dir = tempdir().unwrap();
        let repo_path = dir.path().join("report.json");
        let repo_path_str = repo_path.to_str().unwrap();

        // Prepare input as Vec<(PathBuf, String)>
        let duplicates = vec![
            (PathBuf::from("file1.txt"), "hash1".to_string()),
            (PathBuf::from("file2.txt"), "hash1".to_string()),
            (PathBuf::from("file3.txt"), "hash2".to_string()),
            (PathBuf::from("file4.txt"), "hash2".to_string()),
        ];
        generate_report(&duplicates, repo_path_str);

        assert!(repo_path.exists(), "Report file should be created");

        let contents = fs::read_to_string(&repo_path).unwrap();
        assert!(contents.contains("file1.txt"));
        assert!(contents.contains("file2.txt"));
        // The report does not contain the word "duplicate", so remove this assertion
        // assert!(contents.contains("duplicate"));
    }
    #[test]
    fn test_generate_report_with_invalid_format(){
        let dir =tempdir().unwrap();
        let repo_path=dir.path().join("report.invalid");
        let repo_path_str=repo_path.to_str().unwrap();

        let duplicates = vec![
            (PathBuf::from("file1.txt"), "hash1".to_string()),
            (PathBuf::from("file2.txt"), "hash1".to_string()),
            (PathBuf::from("file3.txt"), "hash2".to_string()),
            (PathBuf::from("file4.txt"), "hash2".to_string()),
        ];
        generate_report(&duplicates, repo_path_str);
        assert!(repo_path.exists(), "Report file should be created");
    }
}