use std::fs;
use std::path::Path;

use crate::cache;
use crate::parallel;
use crate::parse::als::Project;

/// Extracts, parses and creates cache of all *als* files
/// of the following depth:
/// ```
/// projects
/// --------- dir_0
///           ----- als_n.als
/// --------- dir_1
///           ----- als_n.als
/// ```
pub fn run_palsa(dir: &Path) -> Result<Vec<Project>, String> {
    let mut projects: Vec<Project> = Vec::new();

    let mut errors = Vec::new();

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            let project_name = path
                .file_name()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| "Invalid project directory name".to_string())?
                .to_string();

            match parallel::parallel_parse_dir(&project_name, path.to_str().unwrap()) {
                Ok(all_als_data) => {
                    projects.push(Project {
                        name: project_name,
                        als_data: Some(all_als_data),
                    });
                }
                Err(error) => {
                    errors.push(error);
                }
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors.join(", "));
    }

    if let Err(e) = cache::cache(projects.clone()) {
        eprintln!("Error creating cache: {:?}", e);
    }
    let retrieved = cache::retrieve().map_err(|e| format!("Failed to retrieve cache: {}", e))?;

    Ok(retrieved)
}
