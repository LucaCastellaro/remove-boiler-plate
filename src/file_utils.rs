use std::{path::Path, fs};


pub fn delete_file(destination: &str) -> Result<bool, String> {
    let destination_path = Path::new(destination);
    if destination_path.exists() {
        let result = fs::remove_file(destination_path);
        match result {
            Err(err) => return Err(err.to_string()),
            Ok(_) => {}
        }
    }
    return Ok(true);
}