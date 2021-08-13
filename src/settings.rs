use config::{ConfigError, Config, File};
use serde::{Deserialize};
use std::path::PathBuf;
use dirs::home_dir;
use std::process;

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
    pub fn fetch() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        let defaults = Settings::default();
        
        s.merge(File::with_name("Settings"))?;

        let anime_dir = match s.get::<String>("anime_dir") {
            Ok(v) => {
                let mut path = PathBuf::from(v);
                if !path.is_dir() {
                    path = defaults.anime_dir
                }
                path
            },
            Err(_) => defaults.anime_dir,
        };

        let tvshow_dir = match s.get::<String>("tvshow_dir") {
            Ok(v) => {
                let mut path = PathBuf::from(v);
                if !path.is_dir() {
                    path = defaults.tvshow_dir
                }
                path
            },
            Err(_) => defaults.tvshow_dir,
        };

        let movie_dir = match s.get::<String>("movie_dir") {
            Ok(v) => {
                let mut path = PathBuf::from(v);
                if !path.is_dir() {
                    path = defaults.movie_dir
                }
                path
            },
            Err(_) => defaults.movie_dir,
        };

        let autodownload = match s.get_bool("autodownload") {
            Ok(v) => v,
            Err(_) => defaults.autodownload,
        };
        
        Ok(Settings {
            anime_dir,
            tvshow_dir,
            movie_dir,
            autodownload,
        })
    }
}