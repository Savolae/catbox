//! Unofficial library implementing catbox.moe's API in Rust
//!
//! Separates funtionalities into three modules:
//! * `file` for uploading and deleting singular files
//! * `album` for album operations with existing files on Catbox
//! * `litter` for uploading temporary files to Litterbox
//!
//! See <https://catbox.moe/faq.php> for allowed filetypes and content,
//! as well as other questions.
//!
//! Consider donating via <https://www.patreon.com/catbox> to help with server costs.

mod helper;

pub mod album;
pub mod file;
pub mod litter;

static CATBOX_API_URL: &str = "https://catbox.moe/user/api.php";
static LITTER_API_URL: &str = "https://litterbox.catbox.moe/resources/internals/api.php";
static UASTRING: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (CLI tool endorsed on the tools page) - Savolae"
);
