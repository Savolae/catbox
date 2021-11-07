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
