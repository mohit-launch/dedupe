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
