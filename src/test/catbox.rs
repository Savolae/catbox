pub mod album {
    use std::error::Error;

    pub async fn create(
        title: &str,
        desc: &str,
        user_hash: Option<&str>,
        files: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (title, desc, user_hash, files);
        Ok("https://catbox.moe/c/123435".to_string())
    }

    pub async fn delete(short: &str, user_hash: &str) -> Result<String, Box<dyn Error>> {
        let _ = (short, user_hash);
        Ok(String::new())
    }

    pub async fn edit(
        short: &str,
        title: &str,
        desc: &str,
        user_hash: &str,
        files: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (title, desc, user_hash, files);
        Ok(match short.len() > 0 {
            true => format!("https://catbox.moe/c/{}", short),
            false => "No album found for user specified.".to_string(),
        })
    }

    pub async fn add_files(
        short: &str,
        user_hash: &str,
        files: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (user_hash, files);
        Ok(match short.len() > 0 {
            true => format!("https://catbox.moe/c/{}", short),
            false => "No album found for user specified.".to_string(),
        })
    }

    pub async fn remove_files(
        short: &str,
        user_hash: &str,
        files: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = (user_hash, files);
        Ok(match short.len() > 0 {
            true => format!("https://catbox.moe/c/{}", short),
            false => "No album found for user specified.".to_string(),
        })
    }
}

pub mod file {
    use std::{error::Error, fs::File};

    use url::Url;

    pub async fn from_file(
        file_path: &str,
        user_hash: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let _ = user_hash;
        File::open(file_path)?;
        Ok(format!(
            "https://catbox.moe/file.{}",
            file_path.split(".").last().unwrap()
        ))
    }

    pub async fn from_url(url: &str, user_hash: Option<&str>) -> Result<String, Box<dyn Error>> {
        let _ = user_hash;
        Url::parse(url)?;
        Ok(format!(
            "https://catbox.moe/file.{}",
            url.split(".").last().unwrap()
        ))
    }

    pub async fn delete(user_hash: &str, files: Vec<&str>) -> Result<String, Box<dyn Error>> {
        let _ = user_hash;
        let valid = files.into_iter().all(|file| file.len() > 0);
        Ok(match valid {
            true => "Files succesfully deleted.".to_string(),
            false => "File doesn't exist?".to_string(),
        })
    }
}

pub mod litter {
    use std::{error::Error, fs::File};

    pub async fn upload(file_path: &str, time: &str) -> Result<String, Box<dyn Error>> {
        File::open(file_path)?;
        Ok(match vec!["1h", "3h", "24h", "72h"].contains(&time) {
            true => format!(
                "https://catbox.moe/file.{}",
                file_path.split(".").last().unwrap()
            ),
            false => "No expire time specified.".to_string(),
        })
    }
}
