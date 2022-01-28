use std::path::{Path, PathBuf};

pub fn init_dirs() -> Result<PathBuf, String> {
    if let Some(data_dir) = dirs::data_local_dir() {
        let our_dir = data_dir.join(Path::new("satwatch/"));
        if !our_dir.exists() {
            std::fs::create_dir_all(our_dir.clone()).unwrap();
            eprintln!("Creating {}", our_dir.to_str().unwrap());
        }

        Ok(our_dir)
    } else {
        Err("No idea where to save stuff? what?".to_string())
    }
}
