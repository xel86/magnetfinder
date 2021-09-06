use std::path::PathBuf;

pub struct Torrent {
    pub title: String,
    pub magnet: String,
    pub size: String,
    pub seeders: String,
}

pub enum Website {
    Nyaa,
    Piratebay,
}

pub struct Settings {
    pub anime_dir: PathBuf,
    pub tvshow_dir: PathBuf,
    pub movie_dir: PathBuf,
    pub autodownload: bool,
}

pub struct UserParameters {
    pub websites: Vec<Website>,
    pub directory: PathBuf,
    pub search_query: String,
    pub search_depth: u32,
    pub autodownload: bool,
}
