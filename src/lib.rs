mod nyaa;
mod interface;

use std::process;
use crate::interface::{ UserParameters, Website, Media };
use std::env;

#[derive(Debug)]
pub struct Torrent {
    pub title: String,
    pub magnet: String,
    pub size: String,
    pub seeders: String,
}

impl UserParameters {
    fn new(mut args: env::Args) -> () {}
}

pub fn run() {
    let user_parameters = UserParameters::prompt();

    let torrents = match user_parameters.website {
        Website::Nyaa => {
            nyaa::query(&user_parameters.search_query).unwrap_or_else(|err| {
                eprintln!("Error requesting data from nyaa: {}", err);
                process::exit(1);
            })
        },
        Website::Piratebay => process::exit(1),
        Website::All => process::exit(1),
    };

    for torrent in torrents {
        println!("{} - {} - {}", torrent.title, torrent.size, torrent.seeders);
    }

}
