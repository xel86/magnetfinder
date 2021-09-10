use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

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

pub enum Media {
    Anime,
    Movie,
    TVShow,
}

pub struct Settings {
    pub anime_dir: Rc<PathBuf>,
    pub tvshow_dir: Rc<PathBuf>,
    pub movie_dir: Rc<PathBuf>,
    pub autodownload: bool,
}

pub struct UserParameters {
    pub websites: Vec<Website>,
    pub directory: Rc<PathBuf>,
    pub search_query: Arc<String>,
    pub search_depth: u32,
    pub autodownload: bool,
}
