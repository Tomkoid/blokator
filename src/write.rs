use std::process::exit;
use std::fs;

pub fn write_to_file(path: &str, contents: String) {
    fs::write(path, contents).unwrap_or_else(|e| {
        println!(
            "==> Error occurred when writing to {}: {} (Kind: {})",
            path,
            e,
            e.kind()
        );
        exit(1);
    });
}
