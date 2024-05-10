use std::fs;
use std::process::exit;

use crate::logging::get_global_logger;

pub fn write_to_file(path: &str, contents: String) {
    fs::write(path, contents).unwrap_or_else(|e| {
        get_global_logger().log_error(&format!("Error occured when writing to {}: {}", path, e));
        exit(1);
    });
}
