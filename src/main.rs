mod extract;
mod parse;

use extract::extract;
use parse::{parse_als, Track};
use serde_yaml::{to_string, to_value};

// For speed testing
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let extracted_xml_contents = extract("Vast.als".to_string()).unwrap();
    let als_data: Vec<Track> = parse_als(extracted_xml_contents);

    let yaml_data = to_value(&als_data).unwrap();
    let yaml_string = to_string(&yaml_data).unwrap();

    let elapsed_time = start_time.elapsed();
    println!("{}", yaml_string);
    println!("{:#?}", als_data);
    println!("Palsa completed in: {:?}    :)", elapsed_time);
}
