use std::fs;
use std::path::Path;

#[test]
fn file_reading_test(){
    fs::read_to_string(Path::new("./"));
}