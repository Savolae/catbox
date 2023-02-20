use clap::{Args, Parser, Subcommand};

#[derive(Debug, PartialEq, Subcommand)]
pub enum CatboxCommand {
    Upload(Upload),
    Delete(Delete),
    Album(Album),
    Litter(Litter),
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum AlbumCommand {
    Create(AlbumCreate),
    Edit(AlbumEdit),
    Add(AlbumAdd),
    Remove(AlbumRemove),
    Delete(AlbumDelete),
}

#[derive(Parser, Debug)]
#[command(
    about = "Unofficial catbox.moe CLI",
    version,
    propagate_version = true,
    arg_required_else_help(true)
)]
pub struct CatboxArgs {
    #[command(subcommand)]
    pub command: CatboxCommand,
    #[arg(
        global = true,
        short,
        long = "user",
        help = "Catbox API user hash. Read from CATBOX_USER_HASH env variable if not provided."
    )]
    pub user_hash: Option<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Upload to Catbox. Max size 200MB.")]
pub struct Upload {
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(num_args(1..), help = "URLs or paths of the files to upload")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Delete files")]
pub struct Delete {
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(num_args(1..), help = "IDs the files to delete")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Album commands")]
pub struct Album {
    #[command(subcommand)]
    pub album_command: AlbumCommand,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Upload a temporary file to Litterbox. Max size 1GB.")]
pub struct Litter {
    #[arg(short, long, help = "Hours to keep the file", value_parser = valid_hour)]
    pub time: Option<u8>,
    #[arg(num_args(1..), help = "Paths of the files to upload")]
    pub files: Vec<String>,
}

fn valid_hour(hour: &str) -> Result<u8, String> {
    let hour: u8 = hour
        .parse()
        .map_err(|_| format!("{hour} is not a valid number"))?;
    if [1, 12, 24, 72].contains(&hour) {
        Ok(hour)
    } else {
        Err(format!(
            "{hour} is not a valid value (Options: 1, 12, 24, 72"
        ))
    }
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Create a new album")]
pub struct AlbumCreate {
    #[arg(short, long, help = "Title of the album")]
    pub title: String,
    #[arg(short, long, alias = "desc", help = "Description of the album")]
    pub description: Option<String>,
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(num_args(1..), help = "Catbox IDs of the files to add to the album")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Edit an album")]
pub struct AlbumEdit {
    #[arg(short, long, help = "Catbox ID of the album to edit")]
    pub short: String,
    #[arg(short, long, help = "Title of the album")]
    pub title: String,
    #[arg(short, long, alias = "desc", help = "Description of the album")]
    pub description: Option<String>,
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(num_args(1..), help = "Catbox IDs of the files the album should contain")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Add files to an album")]
pub struct AlbumAdd {
    #[arg(short, long, help = "Catbox ID of the album to edit")]
    pub short: String,
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(num_args(1..), help = "Catbox IDs of the files to add to the album")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Remove files from an album")]
pub struct AlbumRemove {
    #[arg(short, long, help = "Catbox ID of the album to edit")]
    pub short: String,
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(num_args(1..), help = "Catbox IDs of the files to remove from the album")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Delete an album")]
pub struct AlbumDelete {
    #[arg(from_global)]
    pub user_hash: Option<String>,
    #[arg(help = "Catbox ID of the album to delete")]
    pub short: String,
}
