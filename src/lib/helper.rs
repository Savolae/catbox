//! Helper functions for handling files
use tokio::{fs::File, io::AsyncReadExt};

use std::error::Error;
use std::path::Path;

/// Read the file and return its contents as a vector of bytes
///
/// # Arguments
///
/// * `file_path` - Path to the file
pub async fn file_contents(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(file_path).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    Ok(contents)
}

/// Strip off the directory and return the file's name and extension
///
/// # Arguments
///
/// * `file_path` - Path to the file
pub fn file_name(file_path: &str) -> String {
    Path::new(file_path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap_or_default()
}
