use crate::parse::als::{AlsData, Project};
use serde_yaml::{from_str, to_string, to_value};
use std::{
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::Path,
};

/// Creates *yaml* files for faster loading as files will not have to be parsed again
pub fn cache(projects: Vec<Project>) -> std::io::Result<()> {
    // Creates the folder for cache if it does not already exist
    fs::create_dir_all("cache/")?;

    for project in projects {
        if let Some(als_data) = project.als_data {
            for als_data in als_data {
                let file_path = format!("cache/{}/{}.yaml", project.name, als_data.name);
                fs::create_dir_all(&format!("cache/{}", project.name))?;

                if !Path::new(&file_path).is_file() {
                    let yaml_data =
                        to_value(&als_data).expect("Failed to convert AlsData to YAML value!");
                    let yaml_string =
                        to_string(&yaml_data).expect("Failed to convert YAML value to string!");

                    let file = File::create(&file_path)?;
                    let mut buf_writer = BufWriter::new(file);
                    buf_writer.write_all(yaml_string.as_bytes())?;
                }
            }
        }
    }
    Ok(())
}

/// Used to retrieve `Project` when `cache` created *yaml* cache files
pub fn retrieve() -> std::io::Result<Vec<Project>> {
    let mut projects = Vec::new();

    for entry in fs::read_dir("cache/")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let project_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid project directory name",
                    )
                })?
                .to_string();

            let mut all_als_data = Vec::new();

            for als_entry in fs::read_dir(&path)? {
                let als_entry = als_entry?;
                let als_path = als_entry.path();
                let is_yaml =
                    als_path.is_file() && als_path.extension().unwrap_or_default() == "yaml";

                if is_yaml {
                    let mut file = File::open(&als_path)?;
                    let mut file_contents = String::new();
                    file.read_to_string(&mut file_contents)?;

                    let als_data: AlsData = from_str(&file_contents)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                    all_als_data.push(als_data);
                }
            }

            projects.push(Project {
                name: project_name,
                als_data: Some(all_als_data),
            });
        }
    }

    Ok(projects)
}
