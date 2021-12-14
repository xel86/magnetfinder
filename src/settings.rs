use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

use config::{Config, ConfigError, File};
use directories::{ProjectDirs, UserDirs};

use crate::Settings;

impl Default for Settings {
    fn default() -> Self {
        if let Some(user_dirs) = UserDirs::new() {
            let downloads_dir = user_dirs
                .download_dir()
                .unwrap_or_else(|| user_dirs.home_dir());

            let downloads_dir: Rc<PathBuf> = Rc::new(PathBuf::from(downloads_dir));
            Settings {
                anime_dir: Rc::clone(&downloads_dir),
                tvshow_dir: Rc::clone(&downloads_dir),
                movie_dir: Rc::clone(&downloads_dir),
                default_directory: Rc::clone(&downloads_dir),
                default_proxy: String::from(""),
                autodownload: false,
                torrent_client: String::from(""),
            }
        } else {
            eprintln!("Error getting home directory");
            process::exit(1);
        }
    }
}

pub struct DownloadDirCache {
    value: Option<Rc<PathBuf>>,
}

impl DownloadDirCache {
    pub fn new(default_path: Result<String, ConfigError>) -> DownloadDirCache {
        let value = match default_path {
            Ok(v) => {
                let path = Rc::new(PathBuf::from(v));
                if !path.is_dir() {
                    return DownloadDirCache { value: None };
                }
                Some(path)
            }
            Err(_) => None,
        };
        DownloadDirCache { value }
    }

    pub fn value(&mut self) -> Rc<PathBuf> {
        match &self.value {
            Some(val) => Rc::clone(val),
            None => {
                if let Some(user_dirs) = UserDirs::new() {
                    let dir = user_dirs
                        .download_dir()
                        .unwrap_or_else(|| user_dirs.home_dir());

                    let dir: Rc<PathBuf> = Rc::new(PathBuf::from(dir));
                    self.value = Some(Rc::clone(&dir));

                    dir
                } else {
                    eprintln!("Error getting home directory");
                    process::exit(1);
                }
            }
        }
    }
}

impl Settings {
    fn validate_path(
        path: Result<String, ConfigError>,
        default: &mut DownloadDirCache,
    ) -> Rc<PathBuf> {
        match path {
            Ok(v) => {
                let mut path = Rc::new(PathBuf::from(v));
                if !path.is_dir() {
                    path = default.value()
                }
                path
            }
            Err(_) => default.value(),
        }
    }

    pub fn fetch() -> Result<Settings, ConfigError> {
        let mut s = Config::default();

        if let Some(proj_dirs) = ProjectDirs::from("", "", "magnetfinder") {
            let config_path = proj_dirs.config_dir();
            let mut config_path = config_path.to_path_buf();
            config_path.push("Settings.toml");

            s.merge(File::from(config_path))?;
        } else {
            eprintln!("Error finding project config directory, falling back to executable path");
            match env::current_exe() {
                Ok(mut exe_path) => {
                    exe_path.pop();
                    exe_path.push("Settings.toml");
                    s.merge(File::from(exe_path))?;
                }
                Err(_) => {
                    s.merge(File::with_name("Settings"))?;
                }
            };
        }

        let mut fallback_dir = DownloadDirCache::new(s.get::<String>("default_directory"));

        let anime_dir = Settings::validate_path(s.get::<String>("anime_dir"), &mut fallback_dir);
        let tvshow_dir = Settings::validate_path(s.get::<String>("tvshow_dir"), &mut fallback_dir);
        let movie_dir = Settings::validate_path(s.get::<String>("movie_dir"), &mut fallback_dir);

        let autodownload = s.get_bool("autodownload").unwrap_or(false);

        let torrent_client = s
            .get::<String>("torrent_client")
            .unwrap_or_else(|_| String::from(""));

        let default_proxy = s
            .get::<String>("default_proxy")
            .unwrap_or_else(|_| String::from(""));

        Ok(Settings {
            anime_dir,
            tvshow_dir,
            movie_dir,
            default_directory: fallback_dir.value(),
            default_proxy,
            autodownload,
            torrent_client,
        })
    }

    pub fn generate_settings_file() -> Result<(), io::Error> {
        let mut file;

        if let Some(proj_dirs) = ProjectDirs::from("", "", "magnetfinder") {
            let config_path = proj_dirs.config_dir();
            let mut config_path = config_path.to_path_buf();

            if !config_path.is_dir() {
                fs::create_dir(&config_path)?;
            }

            config_path.push("Settings.toml");

            file = fs::File::create(config_path)?;
        } else {
            eprintln!("Error finding project config directory, falling back to executable path");
            if let Ok(mut exe_path) = env::current_exe() {
                exe_path.pop();
                exe_path.push("Settings.toml");
                file = fs::File::create(exe_path)?;
            } else {
                file = fs::File::create("Settings.toml")?;
            }
        }

        file.write_all(
            b"# Change directories to where you want each type of media to download to,
# or where default directory is in arg mode

# use absolute paths (/home/user/Downloads/ , C:\\..\\user\\downloads\\ )
# directory priority: anime/tvshow/movie -> default_directory -> /home/user/Downloads

#[ Media Directories (Interactive Mode) ]
anime_dir = \"\"
tvshow_dir = \"\"
movie_dir = \"\"

#[ Default Directory (Arg & Interactive Mode) ]
default_directory = \"\"

#[ Torrent Client ]
# current supported clients are \"deluge\", \"Transmission\", and \"qbittorrent\" (ex: torrent_client = \"deluge\")
torrent_client = \"\"

# Autodownload takes the magnet link selected and
# uses the torrent-client chosen to begin downloading the torrent
autodownload = false

# setting a default proxy allows you to tunnel all scraping from torrent websites through
# this set proxy by default. If using a socks5 proxy, format ip like so: socks5://192.168.1.1:9000
default_proxy = \"\"",
        )?;

        Ok(())
    }
}
