use clap::ArgMatches;

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

async fn parse_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        ("create", Some(sub_cmd)) => create_album(sub_cmd).await,
        ("delete", Some(sub_cmd)) => delete_album(sub_cmd).await,
        ("edit", Some(sub_cmd)) => edit_album(sub_cmd).await,
        ("add", Some(sub_cmd)) => add_to_album(sub_cmd).await,
        ("remove", Some(sub_cmd)) => remove_from_album(sub_cmd).await,
        _ => Ok(matches.usage().to_string()),
    }
}

async fn upload_file<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    let uri = matches.value_of("filepath").unwrap_or_default();
    let env_user = user_hash_from_env();
    let user = matches.value_of("user hash").or(env_user.as_deref());
    Ok(match Path::new(&uri).exists() {
        true => catbox::file::from_file(uri, user).await?,
        false => catbox::file::from_url(uri, user).await?,
    })
}

async fn delete_file<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::file::delete(
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches.values_of("files").unwrap_or_default().collect(),
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
        matches.values_of("files").unwrap_or_default().collect(),
    )
    .await?)
}

async fn delete_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::delete(
        matches.value_of("short").unwrap_or_default(),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
    )
    .await?)
}

async fn edit_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::edit(
        matches.value_of("short").unwrap_or_default(),
        matches.value_of("title").unwrap_or_default(),
        matches.value_of("description").unwrap_or_default(),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches.values_of("files").unwrap_or_default().collect(),
    )
    .await?)
}

async fn add_to_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::add_files(
        matches.value_of("short").unwrap_or_default(),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches.values_of("files").unwrap_or_default().collect(),
    )
    .await?)
}

async fn remove_from_album<'a>(matches: &'a ArgMatches<'static>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(catbox::album::remove_files(
        matches.value_of("short").unwrap_or_default(),
        matches
            .value_of("user hash")
            .unwrap_or(&user_hash_from_env().as_deref().unwrap_or("")),
        matches.values_of("files").unwrap_or_default().collect(),
    )
    .await?)
}
