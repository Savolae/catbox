use std::{env, error::Error, future::Future, path::Path};

use clap::ArgMatches;
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
    match args::get_app().get_matches().subcommand() {
        Some(("upload", sub_cmd)) => upload(sub_cmd).await,
        Some(("delete", sub_cmd)) => delete_file(sub_cmd).await,
        Some(("album", sub_cmd)) => parse_album(sub_cmd).await,
        Some(("litter", sub_cmd)) => litter(sub_cmd).await,
        _ => {
            args::get_app().print_help()?;
            println!(""); // Because print_help does not print a newline at the end
            Err("Invalid command".into())
        },
    }
}

fn user_hash_from_env() -> Option<String> {
    env::var("CATBOX_USER_HASH").ok()
}

fn catbox_url_to_image_name<'a>(url: &'a str) -> &'a str {
    let re = Regex::new(r"^(http[s]?://)?files.catbox.moe/.+").unwrap();
    match re.is_match(url) {
        true => url.split("/").last().unwrap(),
        false => url,
    }
}

fn album_url_to_short<'a>(url: &'a str) -> &'a str {
    let re = Regex::new(r"^(http[s]?://)?catbox.moe/c/.+").unwrap();
    match re.is_match(url) {
        true => url.split("/").last().unwrap(),
        false => url,
    }
}

async fn parse_album<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some(("create", sub_cmd)) => create_album(sub_cmd).await,
        Some(("delete", sub_cmd)) => delete_album(sub_cmd).await,
        Some(("edit", sub_cmd)) => edit_album(sub_cmd).await,
        Some(("add", sub_cmd)) => add_to_album(sub_cmd).await,
        Some(("remove", sub_cmd)) => remove_from_album(sub_cmd).await,
        _ => {
            println!("{}", args::get_app().render_usage());
            Err("Invalid command".into())
        },
    }
}

async fn upload<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let uris: Vec<&str> = matches.values_of("files").unwrap_or_default().collect();
    let (files, rest): (Vec<_>, _) = uris.into_iter().partition(|uri| Path::new(&uri).exists());
    let (urls, rest): (Vec<_>, _) = rest.iter().partition(|uri| Url::parse(uri).is_ok());
    let env_user = user_hash_from_env();
    let user = matches.value_of("user hash").or(env_user.as_deref());
    let print_result = |res| async move { println!("{}", res) };
    tokio::join!(
        rest.into_iter()
            .map(|uri| invalid_uri(uri))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
        urls.into_iter()
            .map(|url| try_upload(file::from_url, url, user))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
        files
            .into_iter()
            .map(|file| try_upload(file::from_file, file, user))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
    );
    Ok(())
}

async fn invalid_uri(uri: &str) -> String {
    format!("Ignoring {}: Not a file or valid URL", uri)
}

async fn try_upload<'a, F, R>(
    upload_funcion: F,
    file_uri: &'a str,
    user_hash: Option<&'a str>,
) -> String
where
    F: FnOnce(&'a str, Option<&'a str>) -> R,
    R: Future<Output = Result<String, Box<dyn Error>>>,
{
    match upload_funcion(file_uri, user_hash).await {
        Ok(res) => res,
        Err(_) => format!("Uploading {} failed.", file_uri),
    }
}

async fn upload_to_litter(filepath: &str, time: &str) -> String {
    match litter::upload(filepath, time).await {
        Ok(res) => res,
        Err(_) => format!("Uploading {} failed.", filepath),
    }
}

async fn delete_file<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let res = file::delete(
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn litter<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let paths: Vec<&str> = matches.values_of("files").unwrap_or_default().collect();
    let (files, rest): (Vec<_>, _) = paths
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
            .map(|file| upload_to_litter(file, matches.value_of("time").unwrap_or("1h")))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_res),
    );
    Ok(())
}

async fn create_album<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let res = album::create(
        matches.value_of("title").unwrap_or_default(),
        matches.value_of("description").unwrap_or_default(),
        matches
            .value_of("user hash")
            .or(user_hash_from_env().as_deref()),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn delete_album<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let res = album::delete(
        album_url_to_short(matches.value_of("short").unwrap_or_default()),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn edit_album<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let res = album::edit(
        album_url_to_short(matches.value_of("short").unwrap_or_default()),
        matches.value_of("title").unwrap_or_default(),
        matches.value_of("description").unwrap_or_default(),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn add_to_album<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let res = album::add_files(
        album_url_to_short(matches.value_of("short").unwrap_or_default()),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
            .collect(),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn remove_from_album<'a>(matches: &'a ArgMatches) -> Result<(), Box<dyn Error>> {
    let res = album::remove_files(
        album_url_to_short(matches.value_of("short").unwrap_or_default()),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
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
        "https://file-examples-com.github.io/uploads/2017/10/file_example_PNG_500kB.png";

    #[tokio::test]
    async fn upload_file() -> Result<(), Box<dyn Error>> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = args::get_app().get_matches_from(vec![
            "catbox",
            "upload",
            "--user",
            "123345",
            file.path().to_str().unwrap(),
        ]);

        upload(args.subcommand_matches("upload").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn upload_url() -> Result<(), Box<dyn Error>> {
        let args = args::get_app()
            .get_matches_from(vec!["catbox", "upload", "--user", "123345", FILE_URL]);

        upload(args.subcommand_matches("upload").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn nonexistant() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
            "catbox",
            "upload",
            "--user",
            "123345",
            "This is not a file or url",
        ]);

        upload(args.subcommand_matches("upload").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn upload_multi() -> Result<(), Box<dyn Error>> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = args::get_app().get_matches_from(vec![
            "catbox",
            "upload",
            "--user",
            "123345",
            file.path().to_str().unwrap(),
            FILE_URL,
            "Something",
            "Something else",
        ]);

        upload(args.subcommand_matches("upload").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn delete_files() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
            "catbox",
            "delete",
            "--user",
            "123345",
            "file.png",
            "another.jpg",
        ]);

        delete_file(args.subcommand_matches("delete").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn album_create() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
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

        parse_album(args.subcommand_matches("album").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn album_add() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
            "catbox", "album", "add", "--user", "123345", "--short", "123asd", "file.png",
        ]);

        parse_album(args.subcommand_matches("album").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn album_remove() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
            "catbox", "album", "remove", "--user", "123345", "--short", "123asd", "file.png",
        ]);

        parse_album(args.subcommand_matches("album").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn album_delete() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
            "catbox", "album", "delete", "--user", "123345", "asd123",
        ]);

        parse_album(args.subcommand_matches("album").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn album_edit() -> Result<(), Box<dyn Error>> {
        let args = args::get_app().get_matches_from(vec![
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

        parse_album(args.subcommand_matches("album").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn upload_litter() -> Result<(), Box<dyn Error>> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = args::get_app().get_matches_from(vec![
            "catbox",
            "litter",
            "--time",
            "1h",
            file.path().to_str().unwrap(),
        ]);

        litter(args.subcommand_matches("litter").unwrap()).await?;

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn invalid_command() {
        let args = args::get_app().get_matches();
        match parse_album(args.subcommand_matches("album").unwrap()).await {
            Ok(()) => assert!(false),
            Err(_) => assert!(true),
        };
    }
}
