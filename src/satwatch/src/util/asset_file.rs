use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn asset_file_name(filename: &str) -> Result<PathBuf, String> {
    let mut path = Path::new(filename).to_path_buf();
    if !path.exists() {
        path = Path::new("./assets/").join(filename);
        if !path.exists() {
            path = Path::new("./src/satwatch/assets/").join(filename);
            if !path.exists() {
                return Err(format!("Could not find {}", filename));
            }
        }
    }

    Ok(path)
}

pub fn asset_file(filename: &str) -> Result<File, String> {
    let path = asset_file_name(filename)?;

    // aaaaaaaaaaaand open!
    // no stairs.
    if let Ok(file) = File::open(path) {
        Ok(file)
    } else {
        Err(format!("Could not open {}", filename))
    }
}

pub fn asset_file_str(filename: &str) -> Result<String, String> {
    let mut file = asset_file(filename)?;
    let mut result = String::new();
    if let Err(e) = file.read_to_string(&mut result) {
        Err(format!("Error reading {}: {}", filename, e.to_string()))
    } else {
        Ok(result)
    }
}
