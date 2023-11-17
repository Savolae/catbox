//! Functions for handling albums through Catbox's API
//!
//! Calls API described at <https://catbox.moe/tools.php>.

use std::error::Error;

use reqwest::Client;

use super::{CATBOX_API_URL, UASTRING};

/// Create a new album
///
/// Returns an URL to the created album
///
/// # Arguments
///
/// * `title` - Album title
/// * `desc` - Album description
/// * `user_hash` - User's account hash, required for deleting or editing. (Optional)
/// * `files` - List of existing files on Catbox to be added to the album
pub async fn create<S: Into<String>>(
    title: S,
    desc: S,
    user_hash: Option<S>,
    files: Vec<S>,
) -> Result<String, Box<dyn Error>> {
    let files: Vec<_> = files.into_iter().map(|file| file.into()).collect();
    let form = [
        ("reqtype", "createalbum"),
        (
            "userhash",
            &user_hash
                .and_then(|hash| Some(hash.into()))
                .unwrap_or_default(),
        ),
        ("title", &title.into()),
        ("desc", &desc.into()),
        ("files", &files.join(" ")),
    ];
    Ok(Client::builder()
        .user_agent(UASTRING)
        .build()
        .unwrap_or_else(|_| Client::new())
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
/// Returns an URL to the album
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `title` - Album title
/// * `desc` - Album description
/// * `files` - List of existing files on Catbox to be included in the album
/// * `user_hash` - User's account hash
pub async fn edit<S: Into<String>>(
    short: S,
    title: S,
    desc: S,
    user_hash: S,
    files: Vec<S>,
) -> Result<String, Box<dyn Error>> {
    let files: Vec<_> = files.into_iter().map(|file| file.into()).collect();
    let form = [
        ("reqtype", "editalbum"),
        ("userhash", &user_hash.into()),
        ("short", &short.into()),
        ("title", &title.into()),
        ("desc", &desc.into()),
        ("files", &files.join(" ")),
    ];
    Ok(Client::builder()
        .user_agent(UASTRING)
        .build()
        .unwrap_or_else(|_| Client::new())
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}

/// Add files to an album
///
/// Returns an URL to the album
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `user_hash` - User's account hash
/// * `files` - List of existing files on Catbox to be added to the album
pub async fn add_files<S: Into<String>>(
    short: S,
    user_hash: S,
    files: Vec<S>,
) -> Result<String, Box<dyn Error>> {
    let files: Vec<_> = files.into_iter().map(|file| file.into()).collect();
    let form = [
        ("reqtype", "addtoalbum"),
        ("short", &short.into()),
        ("userhash", &user_hash.into()),
        ("files", &files.join(" ")),
    ];
    Ok(Client::builder()
        .user_agent(UASTRING)
        .build()
        .unwrap_or_else(|_| Client::new())
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}

/// Remove files from an album
///
/// Returns an URL to the album
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `user_hash` - User's account hash
/// * `files` - List of existing files on Catbox to be removed from the album
pub async fn remove_files<S: Into<String>>(
    short: S,
    user_hash: S,
    files: Vec<S>,
) -> Result<String, Box<dyn Error>> {
    let files: Vec<_> = files.into_iter().map(|file| file.into()).collect();
    let form = [
        ("reqtype", "removefromalbum"),
        ("userhash", &user_hash.into()),
        ("short", &short.into()),
        ("files", &files.join(" ")),
    ];
    Ok(Client::builder()
        .user_agent(UASTRING)
        .build()
        .unwrap_or_else(|_| Client::new())
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}

/// Delete an album
///
/// Returns an empty string
///
/// # Arguments
///
/// * `short` - ID of the album
/// * `user_hash` - User's account hash
pub async fn delete<S: Into<String>>(short: S, user_hash: S) -> Result<String, Box<dyn Error>> {
    let form = [
        ("reqtype", "deletealbum"),
        ("userhash", &user_hash.into()),
        ("short", &short.into()),
    ];
    Ok(Client::builder()
        .user_agent(UASTRING)
        .build()
        .unwrap_or_else(|_| Client::new())
        .post(CATBOX_API_URL)
        .form(&form)
        .send()
        .await?
        .text()
        .await?)
}
