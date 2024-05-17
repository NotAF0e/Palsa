use parallel::parallel_parse;
use serde_yaml::{to_string, to_value};

use std::fs;
use std::path::Path;
use std::time::Instant;

mod extract;
pub mod macros;
mod parallel;
mod parse;

fn main() {
    let dir = "test als files/";
    let file_count = fs::read_dir(Path::new(dir))
        .unwrap() // Handle potential error
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap_or_default() == "als" {
                Some(path)
            } else {
                None
            }
        })
        .count();

    let start_time = Instant::now();
    let all_als_data = parallel_parse(dir);

    // Convert to a yaml string for caching
    let yaml_data = to_value(&all_als_data).unwrap();
    let yaml_string = to_string(&yaml_data).unwrap();

    let elapsed_time = start_time.elapsed();

    // println!("Yaml string:\n{}", yaml_string);
    println!(
        "Palsa completed with {} files in {:?}     :)",
        file_count, elapsed_time
    )
}

// use std::fs;
// use std::path::Path;

// fn main() {
//     let dir_path = Path::new("/path/to/directory");
//     let file_extension = "rs"; // Filter for Rust source files

//     let count = fs::read_dir(dir_path)
//         .unwrap() // Handle potential error
//         .filter_map(|entry| {
//             let path = entry.unwrap().path();
//             if path.is_file() && path.extension().unwrap_or_default() == file_extension {
//                 Some(path)
//             } else {
//                 None
//             }
//         })
//         .count();

//     println!("Number of .{} files: {}", file_extension, count);
// }
