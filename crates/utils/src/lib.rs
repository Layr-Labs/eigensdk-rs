use std::fs;

pub fn read_file(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap();

    content
}
