//! Functions for handling file upload and deletion through Catbox's API
//!
//! Calls API described at <https://catbox.moe/tools.php>.
//!
//! See <https://catbox.moe/faq.php> for allowed filetypes and content.

use reqwest::{
    multipart::{Form, Part},
    Client,
};

use std::error::Error;

use crate::helper::*;
use crate::CATBOX_API_URL;

/// Upload a file to catbox.
///
/// See <https://catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `file_path` - Path to the file to be uploaded
/// * `user_hash` - User's account hash, required for deleting. (Optional)
pub async fn from_file(file_path: &str, user_hash: Option<&str>) -> Result<String, Box<dyn Error>> {
    let file = file_stream(file_path).await?;
    let file_name = file_name(file_path);

    let form = Form::new()
        .text("reqtype", "fileupload")
        .text("userhash", user_hash.unwrap_or_default().to_owned())
        .part("fileToUpload", Part::stream(file).file_name(file_name));

    Ok(Client::new()
        .post(CATBOX_API_URL)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?)
}

/// Upload contents from an URL to catbox
///
/// See <https://catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `url` - URL to file
/// * `user_hash` - User's account hash, required for deleting. (Optional)
pub async fn from_url(url: &str, user_hash: Option<&str>) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "urlupload"),
        ("userhash", user_hash.unwrap_or_default()),
        ("url", url),
    ];
    Ok(Client::new()
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}

/// Delete files
///
/// # Arguments
///
/// * `user_hash` - User's account hash
/// * `files` - Names of the files to be deleted
pub async fn delete(user_hash: &str, files: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "deletefiles"),
        ("userhash", user_hash),
        ("files", &files.join(" ")),
    ];
    Ok(Client::new()
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}
