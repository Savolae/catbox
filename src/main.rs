use std::{env, error::Error, path::Path};

use args::{
    Album, AlbumAdd, AlbumCommand, AlbumCreate, AlbumDelete, AlbumEdit, AlbumRemove, CatboxArgs,
    CatboxCommand, Delete, Litter, Upload,
};

use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use regex::Regex;
use url::Url;

#[cfg(not(test))]
use catbox::{album, file, litter};
#[cfg(test)]
mod test;
#[cfg(test)]
use test::catbox::{album, file, litter};

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match CatboxArgs::try_parse() {
        Ok(args) => match args.command {
            CatboxCommand::Upload(sub_args) => upload(sub_args).await,
            CatboxCommand::Delete(sub_args) => delete_file(sub_args).await,
            CatboxCommand::Album(sub_args) => parse_album(sub_args).await,
            CatboxCommand::Litter(sub_args) => litter(sub_args).await,
        },
        Err(args) => {
            println!("{}", args);
            Ok(())
        },
    }
}

fn user_hash_from_env() -> Option<String> {
    env::var("CATBOX_USER_HASH").ok()
}

fn catbox_url_to_image_name<'a>(url: &'a str) -> String {
    let re = Regex::new(r"^(http[s]?://)?files.catbox.moe/.+").unwrap();
    match re.is_match(url) {
        true => url.split("/").last().unwrap().to_string(),
        false => url.to_string(),
    }
}

fn album_url_to_short<'a>(url: &'a str) -> String {
    let re = Regex::new(r"^(http[s]?://)?catbox.moe/c/.+").unwrap();
    match re.is_match(url) {
        true => url.split("/").last().unwrap().to_string(),
        false => url.to_string(),
    }
}

async fn parse_album(album_args: Album) -> Result<(), Box<dyn Error>> {
    match album_args.album_command {
        AlbumCommand::Create(sub_args) => create_album(sub_args).await,
        AlbumCommand::Delete(sub_args) => delete_album(sub_args).await,
        AlbumCommand::Edit(sub_args) => edit_album(sub_args).await,
        AlbumCommand::Add(sub_args) => add_to_album(sub_args).await,
        AlbumCommand::Remove(sub_args) => remove_from_album(sub_args).await,
    }
}

async fn upload(upload_args: Upload) -> Result<(), Box<dyn Error>> {
    let (files, rest): (Vec<_>, _) = upload_args
        .files
        .into_iter()
        .partition(|uri| Path::new(&uri).exists());
    let (urls, rest): (Vec<_>, _) = rest.iter().partition(|uri| Url::parse(uri).is_ok());
    let env_user = user_hash_from_env();
    let user = upload_args.user_hash.or(env_user);
    let print_result = |res| async move { println!("{}", res) };

    tokio::join!(
        rest.into_iter()
            .map(|uri| invalid_uri(uri.to_string()))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
        urls.into_iter()
            .map(|url| upload_url(url.to_string(), &user))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
        files
            .into_iter()
            .map(|file| upload_file(file, &user))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
    );
    Ok(())
}

async fn invalid_uri(uri: String) -> String {
    format!("Ignoring {}: Not a file or valid URL", uri)
}

async fn upload_file(file: String, user_hash: &Option<String>) -> String {
    match file::from_file(file.clone(), user_hash.as_ref().cloned()).await {
        Ok(res) => res,
        Err(_) => format!("Uploading {} failed.", &file),
    }
}

async fn upload_url(url: String, user_hash: &Option<String>) -> String {
    match file::from_url(&url, user_hash.clone().as_ref()).await {
        Ok(res) => res,
        Err(_) => format!("Uploading {} failed.", url),
    }
}

async fn upload_to_litter(file_path: String, time: u8) -> String {
    match litter::upload(&file_path, time).await {
        Ok(res) => res,
        Err(_) => format!("Uploading {} failed.", file_path),
    }
}

async fn delete_file(delete_args: Delete) -> Result<(), Box<dyn Error>> {
    let res = file::delete(
        delete_args
            .user_hash
            .unwrap_or(user_hash_from_env().as_deref().unwrap_or("").to_string()),
        delete_args
            .files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn litter(litter_args: Litter) -> Result<(), Box<dyn Error>> {
    let (files, rest): (Vec<_>, _) = litter_args
        .files
        .into_iter()
        .partition(|path| Path::new(&path).exists());
    let print_res = |res| async move { println!("{}", res) };
    tokio::join!(
        rest.into_iter()
            .map(|uri| invalid_uri(uri))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_res),
        files
            .into_iter()
            .map(|file| upload_to_litter(file, litter_args.time.unwrap_or(1)))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_res),
    );
    Ok(())
}

async fn create_album(album_create_args: AlbumCreate) -> Result<(), Box<dyn Error>> {
    let res = album::create(
        album_create_args.title,
        album_create_args.description.unwrap_or_default(),
        album_create_args.user_hash.or(user_hash_from_env()),
        album_create_args
            .files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn delete_album(album_delete_args: AlbumDelete) -> Result<(), Box<dyn Error>> {
    let res = album::delete(
        album_url_to_short(&album_delete_args.short),
        album_delete_args
            .user_hash
            .unwrap_or(user_hash_from_env().unwrap_or(String::new())),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn edit_album(album_edit_args: AlbumEdit) -> Result<(), Box<dyn Error>> {
    let res = album::edit(
        album_url_to_short(&album_edit_args.short),
        album_edit_args.title,
        album_edit_args.description.unwrap_or_default(),
        album_edit_args
            .user_hash
            .unwrap_or(user_hash_from_env().unwrap_or(String::new())),
        album_edit_args
            .files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn add_to_album(album_add_args: AlbumAdd) -> Result<(), Box<dyn Error>> {
    let res = album::add_files(
        album_url_to_short(&album_add_args.short),
        album_add_args
            .user_hash
            .unwrap_or(user_hash_from_env().unwrap_or(String::new())),
        album_add_args
            .files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn remove_from_album(album_remove_args: AlbumRemove) -> Result<(), Box<dyn Error>> {
    let res = album::remove_files(
        album_url_to_short(&album_remove_args.short),
        album_remove_args
            .user_hash
            .unwrap_or(user_hash_from_env().unwrap_or(String::new())),
        album_remove_args
            .files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::Builder;

    static FILE_URL: &str =
        "https://file-examples.com/wp-content/uploads/2017/10/file_example_JPG_100kB.jpg";

    #[tokio::test]
    async fn upload_file() -> Result<(), Box<dyn Error>> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "upload",
            "--user",
            "123345",
            file.path().to_str().unwrap(),
        ]);

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn upload_url() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec!["catbox", "upload", "--user", "123345", FILE_URL]);

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn nonexistant() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "upload",
            "--user",
            "123345",
            "This is not a file or url",
        ]);

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn upload_multi() -> Result<(), Box<dyn Error>> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "upload",
            "--user",
            "123345",
            file.path().to_str().unwrap(),
            FILE_URL,
            "Something",
            "Something else",
        ]);

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn delete_files() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "delete",
            "--user",
            "123345",
            "file.png",
            "another.jpg",
        ]);

        if let CatboxCommand::Delete(delete_args) = args.command {
            delete_file(delete_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_create() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "album",
            "create",
            "--desc",
            "A description",
            "--title",
            "My album",
            "--user",
            "123345",
            "file.png",
            "another.jpg",
        ]);

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_add() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox", "album", "add", "--user", "123345", "--short", "123asd", "file.png",
        ]);

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_remove() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox", "album", "remove", "--user", "123345", "--short", "123asd", "file.png",
        ]);

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_delete() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox", "album", "delete", "--user", "123345", "asd123",
        ]);

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_edit() -> Result<(), Box<dyn Error>> {
        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "album",
            "edit",
            "--desc",
            "A description",
            "--title",
            "My Album",
            "--user",
            "123345",
            "--short",
            "asd123",
            "file.png",
            "another.jpg",
        ]);

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn upload_litter() -> Result<(), Box<dyn Error>> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = CatboxArgs::parse_from(vec![
            "catbox",
            "litter",
            "--time",
            "1",
            file.path().to_str().unwrap(),
        ]);

        if let CatboxCommand::Litter(litter_args) = args.command {
            litter(litter_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn invalid_command() {
        match CatboxArgs::try_parse() {
            Ok(args) => {
                if let CatboxCommand::Album(album_args) = args.command {
                    let _ = parse_album(album_args).await;
                }
            },
            Err(_) => panic!("Invalid subcommand"),
        }
    }
}
