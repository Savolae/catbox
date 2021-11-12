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
pub async fn add_files(
    short: &str,
    user_hash: &str,
    files: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
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
pub async fn remove_files(
    short: &str,
    user_hash: &str,
    files: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
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
    let form = [
        ("reqtype", "deletealbum"),
        ("userhash", user_hash),
        ("short", short),
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
mod tests {
    // Files in tests should have different contents as identical files have the same catbox URL
    use std::error::Error;

    use super::{add_files, create, delete, edit, remove_files};

    use crate::file::tests::{delete_files, upload_file};

    #[tokio::test]
    async fn create_and_delete_album() -> Result<(), Box<dyn Error>> {
        let files = vec![
            upload_file("some text").await?,
            upload_file("different text").await?,
        ];
        let file_names: Vec<&str> = files.iter().map(|f| f.split("/").last().unwrap()).collect();

        let res = create(
            "title",
            "desc",
            Some(env!("CATBOX_USER_HASH")),
            file_names.clone(),
        )
        .await?;
        assert!(
            res.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            res
        );

        let res = delete(res.split("/").last().unwrap(), env!("CATBOX_USER_HASH")).await?;
        assert_eq!(res, ""); // Delete does not return anything

        let res = delete_files(file_names).await?;
        assert_eq!(res, "Files successfully deleted.");

        Ok(())
    }

    #[tokio::test]
    async fn add_to_album() -> Result<(), Box<dyn Error>> {
        let files = vec![
            upload_file("first file").await?,
            upload_file("added file").await?,
        ];
        let file_names: Vec<&str> = files.iter().map(|f| f.split("/").last().unwrap()).collect();

        println!("{:?}", files);

        let album = create(
            "title",
            "desc",
            Some(env!("CATBOX_USER_HASH")),
            vec![file_names.first().unwrap()],
        )
        .await?;
        assert!(
            album.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            album
        );

        let res = add_files(
            album.split("/").last().unwrap(),
            env!("CATBOX_USER_HASH"),
            vec![file_names.last().unwrap()],
        )
        .await?;
        assert!(
            res.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            res
        );

        let res = delete(res.split("/").last().unwrap(), env!("CATBOX_USER_HASH")).await?;
        assert_eq!(res, ""); // Delete does not return anything

        let res = delete_files(file_names).await?;
        assert_eq!(res, "Files successfully deleted.");

        Ok(())
    }

    #[tokio::test]
    async fn remove_from_album() -> Result<(), Box<dyn Error>> {
        let files = vec![
            upload_file("to be removed").await?,
            upload_file("keep this one").await?,
        ];
        let file_names: Vec<&str> = files.iter().map(|f| f.split("/").last().unwrap()).collect();

        let album = create(
            "title",
            "desc",
            Some(env!("CATBOX_USER_HASH")),
            file_names.clone(),
        )
        .await?;
        assert!(
            album.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            album
        );

        let res = remove_files(
            album.split("/").last().unwrap(),
            env!("CATBOX_USER_HASH"),
            vec![file_names.last().unwrap()],
        )
        .await?;
        assert!(
            res.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            res
        );

        let res = delete(res.split("/").last().unwrap(), env!("CATBOX_USER_HASH")).await?;
        assert_eq!(res, ""); // Delete does not return anything

        let res = delete_files(file_names).await?;
        assert_eq!(res, "Files successfully deleted.");

        Ok(())
    }

    #[tokio::test]
    async fn edit_album() -> Result<(), Box<dyn Error>> {
        let files = vec![
            upload_file("to be replaced").await?,
            upload_file("new file").await?,
        ];
        let file_names: Vec<&str> = files.iter().map(|f| f.split("/").last().unwrap()).collect();

        let album = create(
            "title",
            "desc",
            Some(env!("CATBOX_USER_HASH")),
            vec![file_names.first().unwrap()],
        )
        .await?;
        assert!(
            album.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            album
        );

        let res = edit(
            album.split("/").last().unwrap(),
            "New title",
            "New desc",
            env!("CATBOX_USER_HASH"),
            vec![file_names.last().unwrap()],
        )
        .await?;
        assert!(
            res.starts_with("https://catbox.moe/"),
            "Catbox returned {}",
            res
        );

        let res = delete(res.split("/").last().unwrap(), env!("CATBOX_USER_HASH")).await?;
        assert_eq!(res, ""); // Delete does not return anything

        let res = delete_files(file_names).await?;
        assert_eq!(res, "Files successfully deleted.");

        Ok(())
    }
}
