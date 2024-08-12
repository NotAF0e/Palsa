use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

/// Extracts the *xml* contents out of the *als* file
pub fn extract(als_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(als_path)?;
    let mut decoder = GzDecoder::new(file);
    let mut xml_contents = String::new();
    decoder.read_to_string(&mut xml_contents)?;

    return Ok(xml_contents);
}

// use flate2::read::GzDecoder;
// use std::fs::File;
// use std::io::{self, Read};

// /// Extracts the *xml* contents out of the *als* file
// pub fn extract(als_path: String) -> Result<String, Box<dyn std::error::Error>> {
//     let mut file = File::open(als_path)?;
//     let mut xml_contents = String::new();

//     if is_gzip_file(&file)? {
//         println!("HELLOW");
//         let mut decoder = GzDecoder::new(file);
//         decoder.read_to_string(&mut xml_contents)?;
//     } else {
//         file.read_to_string(&mut xml_contents);
//     }

//     return Ok(xml_contents);
// }

// // I didnt write this...
// fn is_gzip_file(mut f: &File) -> io::Result<bool> {
//     let mut header = [0u8; 10]; // Gzip header is at least 10 bytes

//     // Read the first 10 bytes
//     f.read_exact(&mut header)?;

//     // Check ID bytes
//     if header[0] != 0x1F || header[1] != 0x8B {
//         return Ok(false);
//     }

//     // Check compression method (should be 8 for DEFLATE)
//     if header[2] != 8 {
//         return Ok(false);
//     }

//     Ok(true)
// }
