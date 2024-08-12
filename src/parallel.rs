// Used for parallelised iteration to extract and parse multiple files at once
use rayon::prelude::*;

use crate::extract;
use crate::parse::als::AlsData;
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

/// Uses [`rayon`]'s `par_iter` and `parallel`'s `find_projects`
/// to find all *als* files in a directory then extracts and parses them in parallel
pub fn parallel_parse_dir(project_name: &str, dir: &str) -> Result<Vec<AlsData>, String> {
    let projects: Vec<String> = match find_projects(dir) {
        Ok(files) => files,
        Err(e) => return Err(e.to_string()),
    };

    let completed_files = Arc::new(Mutex::new(vec![false; projects.len()]));

    let all_als_data: Result<Vec<AlsData>, String> = projects
        .par_iter()
        .enumerate()
        .filter_map(|(i, als_file)| {
            let file_name = Path::new(als_file)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| "Failed to get file stem or convert OsStr to str".to_string())
                .ok()?
                .to_owned();

            if !Path::new(&format!("cache/{}/{}.yaml", project_name, file_name)).is_file() {
                match extract::extract(als_file.clone()).map_err(|e| e.to_string()) {
                    Ok(extracted_xml_contents) => {
                        let als_data = AlsData::parse(file_name, extracted_xml_contents);

                        // Update the completed files
                        let mut completed =
                            completed_files.lock().map_err(|e| e.to_string()).ok()?;
                        completed[i] = true;

                        Some(Ok(als_data))
                    }
                    Err(e) => Some(Err(e)),
                }
            } else {
                // File is already in cache, do nothing
                None
            }
        })
        .collect();

    all_als_data.map(|data| data.into_iter().filter_map(|als| Some(als)).collect())
}

/// Finds all *als* files within a given directory
fn find_projects(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut projects = Vec::new();
    for als_file in fs::read_dir(dir)? {
        let als_file = als_file?;
        let path = als_file.path();

        if path.is_file() && path.extension().unwrap_or_default() == "als" {
            projects.push(path.to_str().unwrap().to_string());
        }
    }
    Ok(projects)
}
