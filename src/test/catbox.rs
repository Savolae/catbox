pub mod album {
    use std::error::Error;

    pub async fn create<S: Into<String>>(
        title: S,
        desc: S,
        user_hash: Option<S>,
        files: Vec<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (title, desc, user_hash, files);
        Ok("https://catbox.moe/c/123435".to_string())
    }

    pub async fn delete<S: Into<String>>(short: S, user_hash: S) -> Result<String, Box<dyn Error>> {
        let _ = (short, user_hash);
        Ok(String::new())
    }

    pub async fn edit<S: Into<String>>(
        short: S,
        title: S,
        desc: S,
        user_hash: S,
        files: Vec<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (title, desc, user_hash, files);
        let short = short.into();
        Ok(match short.len() > 0 {
            true => format!("https://catbox.moe/c/{}", short),
            false => "No album found for user specified.".to_string(),
        })
    }

    pub async fn add_files<S: Into<String>>(
        short: S,
        user_hash: S,
        files: Vec<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (user_hash, files);
        let short = short.into();
        Ok(match short.len() > 0 {
            true => format!("https://catbox.moe/c/{}", short),
            false => "No album found for user specified.".to_string(),
        })
    }

    pub async fn remove_files<S: Into<String>>(
        short: S,
        user_hash: S,
        files: Vec<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (user_hash, files);
        let short = short.into();
        Ok(match short.len() > 0 {
            true => format!("https://catbox.moe/c/{}", short),
            false => "No album found for user specified.".to_string(),
        })
    }
}

pub mod file {
    use std::{error::Error, fs::File};

    use url::Url;

    pub async fn from_file<S: Into<String>>(
        file_path: S,
        user_hash: Option<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = user_hash;
        let file_path = file_path.into();
        File::open(&file_path)?;
        Ok(format!(
            "https://catbox.moe/file.{}",
            file_path.split(".").last().unwrap()
        ))
    }

    pub async fn from_url<S: Into<String>>(
        url: S,
        user_hash: Option<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = user_hash;
        let url = url.into();
        Url::parse(&url)?;
        Ok(format!(
            "https://catbox.moe/file.{}",
            url.split(".").last().unwrap()
        ))
    }

    pub async fn delete<S: Into<String>>(
        user_hash: S,
        files: Vec<S>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = user_hash;
        let valid = files
            .into_iter()
            .map(|file| file.into())
            .all(|file| file.len() > 0);
        Ok(match valid {
            true => "Files succesfully deleted.".to_string(),
            false => "File doesn't exist?".to_string(),
        })
    }
}

pub mod litter {
    use std::{error::Error, fs::File};

    pub async fn upload<S: Into<String>>(file_path: S, time: u8) -> Result<String, Box<dyn Error>> {
        let file_path = file_path.into();
        if ![1, 12, 24, 72].contains(&time) {
            return Ok("Invalid time".to_string());
        }
        File::open(&file_path)?;
        Ok(format!(
            "https://catbox.moe/file.{}",
            file_path.split(".").last().unwrap()
        ))
    }
}
