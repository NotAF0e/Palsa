mod extract;
pub mod macros;
mod parse;
use crate::parse::als::AlsData;

// For yaml conversion for caching
use serde_yaml::{to_string, to_value};

// For speed testing
use std::time::Instant;

fn main() {
    let start_time: Instant = Instant::now();

    let extracted_xml_contents = extract::extract("Vast.als".to_string()).unwrap();
    let als_data: AlsData = AlsData::parse(extracted_xml_contents);

    // Convert to a yaml string for caching
    let yaml_data = to_value(&als_data).unwrap();
    let yaml_string = to_string(&yaml_data).unwrap();

    let elapsed_time = start_time.elapsed();
    println!("{}", yaml_string);
    println!("{:#?}", als_data);
    println!("Palsa completed in: {:?}    :)", elapsed_time);
}
