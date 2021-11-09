//! Functions for handling temporary file upload through Litterbox's API
//!
//! Calls API described at <https://litterbox.catbox.moe/tools.php>.
//!
//! See <https://litterbox.catbox.moe/faq.php> for allowed filetypes and content.

use reqwest::{
    multipart::{Form, Part},
    Client,
};

use std::error::Error;

use crate::helper::*;
use crate::LITTER_API_URL;

/// Upload a temporary file to litterbox.
/// Max size 1GB.
///
/// See <https://litterbox.catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `file_path` - Path to the file to be uploaded
/// * `time` - Time until expiration. Valid values are 1h, 12h, 24h and 72h.
pub async fn upload(file_path: &str, time: &str) -> Result<String, Box<dyn Error>> {
    let file = file_stream(file_path).await?;
    let file_name = file_name(file_path);

    let form = Form::new()
        .text("reqtype", "fileupload")
        .text("time", time.to_owned())
        .part("fileToUpload", Part::stream(file).file_name(file_name));

    Ok(Client::new()
        .post(LITTER_API_URL)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?)
}

#[cfg(test)]
pub mod tests {
    use std::error::Error;

    use crate::file::tests::file_creator;

    use super::*;

    #[tokio::test]
    async fn upload_file() -> Result<(), Box<dyn Error>> {
        let file = file_creator()("Some stuff in a file");
        let res = upload(file.path().to_str().unwrap(), "1h").await?;
        assert!(res.starts_with("https://litter.catbox.moe"));

        Ok(())
    }
}
