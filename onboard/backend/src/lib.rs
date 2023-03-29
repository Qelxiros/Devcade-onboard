use std::error::Error;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod command;

pub mod env {
    use std::env;
    use log::{Level, log};

    /**
     * Get the path to the devcade directory. This is where games are installed.
     * If the value is not set in the environment, it will default to /tmp/devcade.
     */
    pub fn devcade_path() -> String {
        let path = env::var("DEVCADE_PATH");

        match path {
            Ok(path) => path,
            Err(e) => {
                log!(Level::Warn, "Error getting DEVCADE_PATH falling back to '/tmp/devcade': {}", e);
                String::from("/tmp/devcade")
            }
        }
    }

    /**
     * Get the URL of the API. This is where games are downloaded from.
     * If the value is not set in the environment, it will throw a fatal error and panic.
     */
    pub fn api_url() -> String {
        let url = env::var("DEVCADE_API_URL");

        match url {
            Ok(url) => url,
            Err(e) => {
                log!(Level::Error, "Error getting DEVCADE_API_URL: {}", e);
                panic!();
            }
        }
    }
}

/**
 * A game from the Devcade API
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DevcadeGame {
    pub id: String,
    pub author: String,
    pub upload_date: String,
    pub name: String,
    pub hash: String,
    pub description: String,
    pub icon_link: String,
    pub banner_link: String,
}

/**
 * Make a FIFO at the given path. Uses an unsafe call to libc::mkfifo.
 */
fn mkfifo(path: &str) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(path);
    if path.exists() {
        // TODO: Check if it's a FIFO
        return Ok(());
    }
    if !path.parent().unwrap().exists() {
        std::fs::create_dir_all(path.parent().expect("Path has no parent"))?;
    }
    let path = path.to_str().expect("Path is not valid UTF-8");
    let path = std::ffi::CString::new(path)?;
    unsafe {
        libc::mkfifo(path.as_ptr(), 0o644);
    }
    Ok(())
}

/**
 * Open the file at the given path as a read-only FIFO pipe.
 */
pub fn open_read(path_str: &str, create: bool) -> Result<std::fs::File, Box<dyn Error>> {
    let path = std::path::Path::new(path_str);
    if !path.exists() && !create {
        return Err("Path does not exist".into());
    }
    if !path.parent().expect("Path has no parent").exists() && create {
        std::fs::create_dir_all(path.parent().expect("Path has no parent"))?;
    }
    if !path.exists() && create {
        mkfifo(path_str).expect("Failed to create FIFO");
    }
    if !path.is_file() {
        return Err("Path is not a file".into());
    }
    let file = std::fs::OpenOptions::new().read(true).open(path)?;
    Ok(file)
}

/**
 * Open the file at the given path as a write-only FIFO pipe.
 */
pub fn open_write(path_str: &str, create: bool) -> Result<std::fs::File, Box<dyn Error>> {
    let path = std::path::Path::new(path_str);
    if !path.exists() && !create {
        return Err("Path does not exist".into());
    }
    if !path.parent().expect("Path has no parent").exists() && create {
        std::fs::create_dir_all(path.parent().expect("Path has no parent"))?;
    }
    if !path.exists() && create {
        mkfifo(path_str).expect("Failed to create FIFO");
    }
    if !path.is_file() {
        return Err("Path is not a file".into());
    }
    let file = std::fs::OpenOptions::new().write(true).open(path)?;
    Ok(file)
}