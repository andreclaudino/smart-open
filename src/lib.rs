extern crate flate2;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::string::String;
use flate2::read::GzDecoder;

use std::io::Read;

pub fn smart_open(filepath: &str) -> std::io::Result<String> {
    // TODO: make the function more modular.
    let path = Path::new(filepath);
    let content_type = match path.extension() {
        None => panic!("Paths without extension is not allowed!!"),
        Some(os_str) => {
            match os_str.to_str() {
                Some("txt") => "text",
                Some("gz") => "gz",
                Some(e) => panic!("{} case is not implemented yet. Please contact the developers!", e),
                None => panic!("None has seeped in somehow. Please contact developers."),
            }
        }
    };
    let file_handler = File::open(&path)?;
    let mut buf_reader = BufReader::new(file_handler);
    let mut contents = String::new();
    if content_type == "gz" {
        let mut reader = GzDecoder::new(buf_reader);
        reader.read_to_string(&mut contents).unwrap();
    } else if content_type == "text" {
        buf_reader.read_to_string(&mut contents)?;
    } else {
        panic!("Content type {} is not allowed.", content_type);
    }
    Ok(contents.trim().to_string())
}