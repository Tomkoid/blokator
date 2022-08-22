use std::fs;

pub fn write_to_file(path: &str, contents: String) {
    fs::write(path, contents).unwrap();
}
