use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;

    let mut file_output = String::new();

    file.read_to_string(&mut file_output)?;
    
    Ok(file_output)
}
