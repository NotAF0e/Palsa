use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

pub fn extract(als_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(als_path)?;
    let mut decoder = GzDecoder::new(file);
    let mut xml_contents = String::new();
    decoder.read_to_string(&mut xml_contents)?;

    return Ok(xml_contents);
}
