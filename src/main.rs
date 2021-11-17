use clap::ArgMatches;
use futures::future::join_all;
use regex::Regex;

use std::{env, error::Error, path::Path};

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(match args::get_app().get_matches().subcommand() {
        ("upload", Some(sub_cmd)) => upload_file(sub_cmd).await?,
        ("delete", Some(sub_cmd)) => delete_file(sub_cmd).await?,
        ("album", Some(sub_cmd)) => parse_album(sub_cmd).await?,
        ("litter", Some(sub_cmd)) => litter(sub_cmd).await?,
        _ => {
            args::get_app().print_help()?;
            println!(""); // Because print_help does not print a newline at the end
        }
    })
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

async fn parse_album<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    Ok(match matches.subcommand() {
        ("create", Some(sub_cmd)) => create_album(sub_cmd).await?,
        ("delete", Some(sub_cmd)) => delete_album(sub_cmd).await?,
        ("edit", Some(sub_cmd)) => edit_album(sub_cmd).await?,
        ("add", Some(sub_cmd)) => add_to_album(sub_cmd).await?,
        ("remove", Some(sub_cmd)) => remove_from_album(sub_cmd).await?,
        _ => println!("{}", matches.usage().to_string()),
    })
}

async fn upload_file<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let uris: Vec<&str> = matches.values_of("files").unwrap_or_default().collect();
    let (files, urls): (Vec<_>, _) = uris.into_iter().partition(|uri| Path::new(&uri).exists());
    let env_user = user_hash_from_env();
    let user = matches.value_of("user hash").or(env_user.as_deref());
    let url_futures: Vec<_> = urls
        .into_iter()
        .map(|url| upload_url_and_print(url, user))
        .collect();
    let file_futures: Vec<_> = files
        .into_iter()
        .map(|file| upload_file_and_print(file, user))
        .collect();
    join_all(url_futures).await;
    join_all(file_futures).await;
    Ok(())
}

async fn upload_file_and_print(file: &str, user: Option<&str>) {
    match catbox::file::from_file(file, user).await {
        Ok(res) => println!("{}", res),
        Err(_) => println!("Uploading {} failed.", file),
    }
}

async fn upload_url_and_print(url: &str, user: Option<&str>) {
    match catbox::file::from_url(url, user).await {
        Ok(res) => println!("{}", res),
        Err(_) => println!("Uploading {} failed.", url),
    }
}

async fn delete_file<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::file::delete(
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

async fn litter<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::litter::upload(
        matches.value_of("filepath").unwrap_or_default(),
        matches.value_of("time").unwrap_or("1h"),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn create_album<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::album::create(
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

async fn delete_album<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::album::delete(
        album_url_to_short(matches.value_of("short").unwrap_or_default()),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
    )
    .await?;
    println!("{}", res);
    Ok(())
}

async fn edit_album<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::album::edit(
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

async fn add_to_album<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::album::add_files(
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

async fn remove_from_album<'a>(matches: &'a ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    let res = catbox::album::remove_files(
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
