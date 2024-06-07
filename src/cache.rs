use crate::parse::als::AlsData;
use serde_yaml::{from_str, to_string, to_value};
use std::{
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::Path,
};

/// Creates *yaml* files for faster loading as files will not have to be parsed again
pub fn cache(all_als_data: Vec<AlsData>) -> std::io::Result<()> {
    // Creates the folder for cache if it does not already exist
    fs::create_dir_all("cache/")?;

    for als_data in all_als_data {
        if !Path::new(&format!("cache/{}.yaml", als_data.name)).is_file() {
            let yaml_data = to_value(&als_data).expect("Failed to convert AlsData to YAML value!");
            let yaml_string =
                to_string(&yaml_data).expect("Failed to convert YAML value to string!");

            let file_path = format!("cache/{}.yaml", als_data.name);
            let file = File::create(&file_path)?;
            let mut buf_writer = BufWriter::new(file);
            buf_writer.write_all(yaml_string.as_bytes())?;
        }
    }
    Ok(())
}

/// Used to retrieve `AlsData` when `cache` created *yaml* cache files
pub fn retrieve() -> std::io::Result<Vec<AlsData>> {
    let mut all_als_data: Vec<AlsData> = Vec::new();

    for entry in fs::read_dir("cache/")? {
        let entry = entry?;
        let path = entry.path();

        let is_yaml = path.is_file() && path.extension().unwrap_or_default() == "yaml";
        if is_yaml {
            let mut file = File::open(&path)?;
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents)?;

            // Deserialize the YAML string into AlsData
            let als_data: AlsData = from_str(&file_contents)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            all_als_data.push(als_data);
        }
    }

    Ok(all_als_data)
}
