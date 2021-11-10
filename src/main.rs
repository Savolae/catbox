use clap::ArgMatches;
use regex::Regex;

use std::env;
use std::path::Path;

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = args::get_app().get_matches();
    let result = match matches.subcommand() {
        ("upload", Some(sub_cmd)) => upload_file(sub_cmd).await?,
        ("delete", Some(sub_cmd)) => delete_file(sub_cmd).await?,
        ("album", Some(sub_cmd)) => parse_album(sub_cmd).await?,
        ("litter", Some(sub_cmd)) => litter(sub_cmd).await?,
        _ => {
            args::get_app().print_help()?;
            String::new()
        }
    };

    println!("{}", result);
    Ok(())
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

async fn parse_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(match matches.subcommand() {
        ("create", Some(sub_cmd)) => create_album(sub_cmd).await?,
        ("delete", Some(sub_cmd)) => delete_album(sub_cmd).await?,
        ("edit", Some(sub_cmd)) => edit_album(sub_cmd).await?,
        ("add", Some(sub_cmd)) => add_to_album(sub_cmd).await?,
        ("remove", Some(sub_cmd)) => remove_from_album(sub_cmd).await?,
        _ => matches.usage().to_string(),
    })
}

async fn upload_file<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    let files: Vec<&str> = matches.values_of("files").unwrap_or_default().collect();
    let env_user = user_hash_from_env();
    let user = matches.value_of("user hash").or(env_user.as_deref());
    let mut results = vec![];
    for uri in files {
        results.push(match Path::new(&uri).exists() {
            true => catbox::file::from_file(uri, user).await?,
            false => catbox::file::from_url(uri, user).await?,
        });
    }

    Ok(results.join("\n"))
}

async fn delete_file<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::file::delete(
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
            .collect(),
    )
    .await?)
}

async fn litter<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::litter::upload(
        matches.value_of("filepath").unwrap_or_default(),
        matches.value_of("time").unwrap_or("1h"),
    )
    .await?)
}

async fn create_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::create(
        matches.value_of("title").unwrap_or_default(),
        matches.value_of("description").unwrap_or_default(),
        matches.value_of("user hash").or(user_hash_from_env().as_deref()),
        matches
            .values_of("files")
            .unwrap_or_default()
            .map(|file| catbox_url_to_image_name(file))
            .collect(),
    )
    .await?)
}

async fn delete_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::delete(
        album_url_to_short(matches.value_of("short").unwrap_or_default()),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
    )
    .await?)
}

async fn edit_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::edit(
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
    .await?)
}

async fn add_to_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::add_files(
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
    .await?)
}

async fn remove_from_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::remove_files(
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
    .await?)
}
