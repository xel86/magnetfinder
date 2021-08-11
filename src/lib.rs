mod nyaa;
mod interface;

use std::process;
use crate::interface::{ UserParameters, Website };
use comfy_table::Table;

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

    let mut table = Table::new();
    let table = interface::update_torrent_table(&mut table, &torrents);
    println!("{}", table);

    let magnets = interface::get_selected_magnets(&torrents);
    for m in magnets {
        println!("{}", m);
    }
}
