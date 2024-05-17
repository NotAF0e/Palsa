use crate::extract;

use crate::parse::als::AlsData;
use colored::*;
use rayon::prelude::*;
use std::{
    fs,
    sync::{Arc, Mutex},
};

/// Uses [`rayon`], `find_als_files` and `print_progress`
/// to find all *als* files in a directory then parses them in parallel
pub fn parallel_parse(dir: &str) -> Vec<AlsData> {
    let als_files: Vec<String> = find_als_files(dir).unwrap();
    let completed_files = Arc::new(Mutex::new(vec![false; als_files.len()]));

    let all_als_data: Vec<AlsData> = als_files
        .par_iter()
        .enumerate()
        .map(|(i, als_file)| {
            let extracted_xml_contents = extract::extract(als_file.clone()).unwrap();
            let als_data = AlsData::parse(extracted_xml_contents);

            // Update the completed files
            let mut completed = completed_files.lock().unwrap();
            completed[i] = true;

            print!("\x1B[2J\x1B[1;1H"); // Clears the screen by scrolling the terminal
            println!("//// ------------------------ //// \n");
            println!("{}", print_progress(&als_files, &completed));

            als_data
        })
        .collect();

    return all_als_data;
}

/// Finds all .als files within the given directory.
fn find_als_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut als_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "als" {
            als_files.push(path.to_str().unwrap().to_string());
        }
    }
    Ok(als_files)
}

/// Prints the progress bar with tickboxes
fn print_progress(als_files: &Vec<String>, completed: &Vec<bool>) -> String {
    let mut output = String::new();
    for (i, file) in als_files.iter().enumerate() {
        output.push_str(&format!(
            "  [{}] {}",
            if completed[i] {
                "✓".green()
            } else {
                "X".red()
            },
            if completed[i] {
                file.green()
            } else {
                file.red()
            },
        ));
        output.push('\n');
    }
    output
}
