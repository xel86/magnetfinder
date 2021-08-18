use config::{ConfigError, Config, File};
use serde::{Deserialize};
use dirs::home_dir;

use std::path::PathBuf;
use std::process;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub anime_dir: PathBuf,
    pub tvshow_dir: PathBuf,
    pub movie_dir: PathBuf,
    pub autodownload: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let mut downloads_dir = match home_dir() {
            Some(p) => p,
            None => {
                eprintln!("Error getting home directory");
                process::exit(1);
            }
        };
        downloads_dir.push("Downloads/");
        Settings {
            anime_dir: downloads_dir.clone(),
            tvshow_dir: downloads_dir.clone(),
            movie_dir: downloads_dir.clone(),
            autodownload: false,
        }
    }
}

impl Settings {
    pub fn get_downloads_dir() -> PathBuf {
        let mut downloads_dir = match home_dir() {
            Some(p) => p,
            None => {
                eprintln!("Error getting home directory");
                process::exit(1);
            }
        };
        downloads_dir.push("Downloads/");

        downloads_dir
    }
    pub fn fetch() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        let download_dir = Settings::get_downloads_dir();
        
        s.merge(File::with_name("Settings"))?;

        let anime_dir = match s.get::<String>("anime_dir") {
            Ok(v) => {
                let mut path = PathBuf::from(v);
                if !path.is_dir() {
                    path = download_dir.clone()
                }
                path
            },
            Err(_) => download_dir.clone(),
        };

        let tvshow_dir = match s.get::<String>("tvshow_dir") {
            Ok(v) => {
                let mut path = PathBuf::from(v);
                if !path.is_dir() {
                    path = download_dir.clone()
                }
                path
            },
            Err(_) => download_dir.clone(),
        };

        let movie_dir = match s.get::<String>("movie_dir") {
            Ok(v) => {
                let mut path = PathBuf::from(v);
                if !path.is_dir() {
                    path = download_dir.clone()
                }
                path
            },
            Err(_) => download_dir.clone(),
        };

        let autodownload = match s.get_bool("autodownload") {
            Ok(v) => v,
            Err(_) => false,
        };
        
        Ok(Settings {
            anime_dir,
            tvshow_dir,
            movie_dir,
            autodownload,
        })
    }

    pub fn generate_settings_file() -> Result<(), io::Error>{
        let mut file = fs::File::create("Settings.toml")?;

        file.write_all(
b"# Change directories to where you want each type of media to download to
# The default directory is your downloads folder
# use absolute paths (/home/user/Downloads/ , C:\\..\\user\\downloads\\ )
anime_dir = \"\"
tvshow_dir = \"\"
movie_dir = \"\"

# Autodownload takes the magnet link selected and
# uses the torrent-client chosen to begin downloading the torrent
autodownload = false"
        )?;

        Ok(())
    }
}