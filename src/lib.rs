mod nyaa;
mod interface;

use std::process;
use crate::interface::{ UserParameters, Website };

#[derive(Debug)]
pub struct Torrent {
    pub title: String,
    pub magnet: String,
    pub size: String,
    pub seeders: String,
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

    interface::display_torrent_table(&torrents);

    let magnets = interface::prompt_magnet_selection(&torrents);
    for m in magnets {
        println!("{}", m);
    }
}
