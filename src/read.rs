use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path);
    
    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut file_output = String::new();

    match file.read_to_string(&mut file_output) {
        Ok(_) => Ok(file_output),
        Err(e) => return Err(e),
    }
}
