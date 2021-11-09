//! Functions for handling albums through Catbox's API
//!
//! Calls API described at <https://catbox.moe/tools.php>.

use reqwest::Client;

use std::error::Error;

use super::CATBOX_API_URL;

/// Create a new album
///
/// # Arguments
///
/// * `title` - Album title
/// * `desc` - Album description
/// * `user_hash` - User's account hash, required for deleting or editing. (Optional)
/// * `files` - List of existing files on Catbox to be added to the album
pub async fn create(
    title: &str,
    desc: &str,
    user_hash: Option<&str>,
    files: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "createalbum"),
        ("userhash", user_hash.unwrap_or_default()),
        ("title", title),
        ("desc", desc),
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

/// Edit an album
///
/// **NOTE:** Old album will be "overwritten" with the new information.
/// Include everything you want the album to have in the call.
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `title` - Album title
/// * `desc` - Album description
/// * `files` - List of existing files on Catbox to be included in the album
/// * `user_hash` - User's account hash
pub async fn edit(
    short: &str,
    title: &str,
    desc: &str,
    user_hash: &str,
    files: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "editalbum"),
        ("userhash", user_hash),
        ("short", short),
        ("title", title),
        ("desc", desc),
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

/// Add files to an album
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `user_hash` - User's account hash
/// * `files` - List of existing files on Catbox to be added to the album
pub async fn add_files(short: &str, user_hash: &str, files: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "addtoalbum"),
        ("short", short),
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

/// Remove files from an album
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `user_hash` - User's account hash
/// * `files` - List of existing files on Catbox to be removed from the album
pub async fn remove_files(short: &str, user_hash: &str, files: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "removefromalbum"),
        ("userhash", user_hash),
        ("short", short),
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

/// Delete an album
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `user_hash` - User's account hash
pub async fn delete(short: &str, user_hash: &str) -> Result<String, Box<dyn Error>> {
    let form = [("reqtype", "deletealbum"), ("userhash", user_hash), ("short", short)];
    Ok(Client::new()
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{create, delete};

    use crate::file::tests::{delete_files, upload_file};

    #[tokio::test]
    async fn create_and_delete_album() -> Result<(), Box<dyn Error>> {
        let files = vec![upload_file("some text").await?, upload_file("different text").await?];
        let file_names: Vec<&str> = files.iter().map(|f| f.split("/").last().unwrap()).collect();
        println!("Uploaded files {:?}", files);

        let res = create("title", "desc", Some(env!("CATBOX_USER_HASH")), file_names.clone()).await?;
        assert!(res.starts_with("https://catbox.moe/"));

        let res = delete(res.split("/").last().unwrap(), env!("CATBOX_USER_HASH")).await?;
        assert_eq!(res, ""); // Delete does not return anything

        let res = delete_files(file_names).await?;
        assert_eq!(res, "Files successfully deleted.");

        Ok(())
    }
}
