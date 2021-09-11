use std::path::PathBuf;
use std::process;
use std::fs;
use std::io::{self, Write};
use std::rc::Rc;

use config::{ConfigError, Config, File};
use dirs::home_dir;

use crate::Settings;

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

        let downloads_dir: Rc<PathBuf> = Rc::new(downloads_dir);
        Settings {
            anime_dir: Rc::clone(&downloads_dir),
            tvshow_dir: Rc::clone(&downloads_dir),
            movie_dir: Rc::clone(&downloads_dir),
            autodownload: false,
        }
    }
}

pub struct DownloadDirCache {
    value: Option<Rc<PathBuf>>,
}

impl DownloadDirCache {
    pub fn new() -> DownloadDirCache {
        DownloadDirCache {
            value: None,
        }
    }

    pub fn value(&mut self) -> Rc<PathBuf> {
        match &self.value {
            Some(val) => Rc::clone(&val),
            None => {
                let mut dir = match home_dir() {
                    Some(p) => p,
                    None => {
                        eprintln!("Error getting home directory");
                        process::exit(1);
                    }
                };
                dir.push("Downloads/");
                
                let dir: Rc<PathBuf> = Rc::new(dir);
                self.value = Some(Rc::clone(&dir));
                dir
            }
        }
    }
}

impl Settings {
    fn validate_set_path(path: Result<String, ConfigError>, default: &mut DownloadDirCache) -> Rc<PathBuf> {
        match path {
            Ok(v) => {
                let mut path = Rc::new(PathBuf::from(v));
                if !path.is_dir() {
                    path = default.value()
                }
                path
            },
            Err(_) => default.value(),
        }
    }

    pub fn fetch() -> Result<Settings, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("Settings"))?;

        let mut fallback_dir = DownloadDirCache::new();

        let anime_dir = Settings::validate_set_path(s.get::<String>("anime_dir"), &mut fallback_dir);
        let tvshow_dir = Settings::validate_set_path(s.get::<String>("tvshow_dir"), &mut fallback_dir);
        let movie_dir = Settings::validate_set_path(s.get::<String>("movie_dir"), &mut fallback_dir);

        let autodownload = s.get_bool("autodownload").unwrap_or(false);
        
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
