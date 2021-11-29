//! Helper functions for handling files
use std::{error::Error, path::Path};

use reqwest::Body;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

/// Return a Body wrapping a stream to the file's contents
///
/// # Arguments
///
/// * `file_path` - Path to the file
pub async fn file_stream(file_path: &str) -> Result<Body, Box<dyn Error>> {
    Ok(Body::wrap_stream(ReaderStream::new(
        File::open(file_path).await?,
    )))
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
