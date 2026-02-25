use std::{fs, io::Result, path::PathBuf};

use directories::ProjectDirs;

pub fn get_file_path() -> PathBuf {
    let file_path =
        ProjectDirs::from("com", "something", "pwngr").expect("Could not get directory");

    let file_dir = file_path.config_dir();
    file_dir.join("vault.json")
}

pub fn is_initialized() -> bool {
    get_file_path().exists()
}

pub fn create_file() -> Result<()> {
    let path = get_file_path();
    // check if storage file already exists
    if path.exists() {
        println!("Storage file already initialized.");
        return Ok(());
    }

    // check and create dir if does not exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // create initial storage value in file
    // currently json move to a better one later
    let initial_content = r#"{"entries":[]}"#;
    fs::write(path, initial_content)?;

    println!("Storage file initialized.");

    Ok(())
}
