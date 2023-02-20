//! Functions for handling file upload and deletion through Catbox's API
//!
//! Calls API described at <https://catbox.moe/tools.php>.
//!
//! See <https://catbox.moe/faq.php> for allowed filetypes and content.

use std::error::Error;

use reqwest::{
    multipart::{Form, Part},
    Client,
};

use crate::{helper::*, CATBOX_API_URL};

/// Upload a file to catbox.
///
/// Returns an URL to the file
///
/// See <https://catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `file_path` - Path to the file to be uploaded
/// * `user_hash` - User's account hash, required for deleting. (Optional)
pub async fn from_file<S: Into<String>>(
    file_path: S,
    user_hash: Option<S>,
) -> Result<String, Box<dyn Error>> {
    let file_path = file_path.into();
    let file = file_stream(&file_path).await?;
    let file_name = file_name(&file_path);

    let form = Form::new()
        .text("reqtype", "fileupload")
        .text(
            "userhash",
            user_hash
                .and_then(|hash| Some(hash.into()))
                .unwrap_or_default(),
        )
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
/// Returns an URL to the file
///
/// See <https://catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `url` - URL to file
/// * `user_hash` - User's account hash, required for deleting. (Optional)
pub async fn from_url<S: Into<String>>(
    url: S,
    user_hash: Option<S>,
) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "urlupload"),
        (
            "userhash",
            &user_hash
                .and_then(|hash| Some(hash.into()))
                .unwrap_or_default(),
        ),
        ("url", &url.into()),
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
/// Returns "Files successfully deleted." on success
///
/// # Arguments
///
/// * `user_hash` - User's account hash
/// * `files` - Names of the files to be deleted
pub async fn delete<S: Into<String>>(
    user_hash: S,
    files: Vec<S>,
) -> Result<String, Box<dyn Error>> {
    let files: Vec<_> = files.into_iter().map(|file| file.into()).collect();
    let form = [
        ("reqtype", "deletefiles"),
        ("userhash", &user_hash.into()),
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
