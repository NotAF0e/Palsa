use crate::{extract, parse::als};

use crate::parse::als::AlsData;
use rayon::prelude::*;
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

/// Uses [`rayon`] and `find_als_files`
/// to find all *als* files in a directory then parses them in parallel
pub fn parallel_parse(dir: &str) -> Result<Vec<AlsData>, String> {
    let als_files: Vec<String> = match find_als_files(dir) {
        Ok(files) => files,
        Err(e) => return Err(e.to_string()),
    };

    let completed_files = Arc::new(Mutex::new(vec![false; als_files.len()]));

    let all_als_data: Result<Vec<AlsData>, String> = als_files
        .par_iter()
        .enumerate()
        .map(|(i, als_file)| {
            let file_name = Path::new(als_file)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| "Failed to get file stem or convert OsStr to str".to_string())?
                .to_owned();

            if !Path::new(&format!("cache/{}.yaml", file_name)).is_file() {
                let extracted_xml_contents = extract::extract(als_file.clone())
                    .map_err(|e| e.to_string())?;
                let als_data = AlsData::parse(file_name, extracted_xml_contents);

                // Update the completed files
                let mut completed = completed_files.lock().map_err(|e| e.to_string())?;
                completed[i] = true;

                Ok(als_data)
            } else {
                Err("File already exists in cache".to_string())
            }
        })
        .collect();

    all_als_data.map(|data| data.into_iter().filter_map(|als|Some(als)).collect())
}

/// Finds all .als files within the given directory.
fn find_als_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut als_files = Vec::new();
    for als_file in fs::read_dir(dir)? {
        let als_file = als_file?;
        let path = als_file.path();

        if path.is_file() && path.extension().unwrap_or_default() == "als" {
            als_files.push(path.to_str().unwrap().to_string());
        }
    }
    Ok(als_files)
}
