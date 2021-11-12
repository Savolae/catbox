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

#[cfg(test)]
pub mod tests {
    // Note: All tests assume that an user hash is set in env

    use reqwest;
    use sha2::{Digest, Sha256};
    use tempfile::{Builder, NamedTempFile};

    use std::io::prelude::*;

    use std::error::Error;

    use super::{delete, from_file, from_url};

    static FILE_CONTENT: &str = "This is a test file";
    static IMAGE_URL: &str =
        "https://file-examples-com.github.io/uploads/2017/10/file_example_PNG_500kB.png";

    pub fn file_creator() -> impl Fn(&str) -> NamedTempFile {
        move |content: &str| {
            let mut file = Builder::new().tempfile().unwrap();
            write!(file, "{}", content).unwrap();
            file
        }
    }

    pub async fn upload_file(content: &str) -> Result<String, Box<dyn Error>> {
        let file = file_creator();
        from_file(
            file(content).path().to_str().unwrap(),
            Some(env!("CATBOX_USER_HASH")),
        )
        .await
    }

    pub async fn delete_files(files: Vec<&str>) -> Result<String, Box<dyn Error>> {
        Ok(delete(env!("CATBOX_USER_HASH"), files).await?)
    }

    #[tokio::test]
    async fn upload_and_delete_file() -> Result<(), Box<dyn Error>> {
        let file = file_creator()(FILE_CONTENT);
        let res = from_file(
            file.path().to_str().unwrap(),
            Some(env!("CATBOX_USER_HASH")),
        )
        .await?;
        assert!(
            res.starts_with("https://files.catbox.moe/"),
            "Catbox returned {:?}!",
            res
        );

        let download = reqwest::get(&res).await?;
        let original_hash = Sha256::new().chain(FILE_CONTENT).finalize();
        let download_hash = Sha256::new().chain(download.text().await?).finalize();

        assert_eq!(
            original_hash, download_hash,
            "Downloaded file did not match uploaded file!"
        );

        let file_name = res.split("/").last().unwrap();
        let res = delete_files(vec![file_name]).await?;
        assert_eq!(
            res, "Files successfully deleted.",
            "Catbox returned {:?}!",
            res
        );

        Ok(())
    }

    #[tokio::test]
    async fn upload_and_delete_url() -> Result<(), Box<dyn Error>> {
        let res = from_url(IMAGE_URL, Some(env!("CATBOX_USER_HASH"))).await?;
        assert!(
            res.starts_with("https://files.catbox.moe/"),
            "Catbox returned {:?}!",
            res
        );

        let download = reqwest::get(&res).await?;
        let original = reqwest::get(IMAGE_URL).await?;
        let original_hash = Sha256::new().chain(original.text().await?).finalize();
        let download_hash = Sha256::new().chain(download.text().await?).finalize();

        assert_eq!(
            original_hash, download_hash,
            "Downloaded file did not match uploaded file!"
        );

        let file_name = res.split("/").last().unwrap();
        let res = delete_files(vec![file_name]).await?;
        assert_eq!(
            res, "Files successfully deleted.",
            "Catbox returned {:?}!",
            res
        );

        Ok(())
    }
}
