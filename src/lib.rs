mod nyaa;
mod piratebay;
mod interface;
mod settings;

use std::process;
use std::path::PathBuf;
use std::cmp::Reverse;
use clap::ArgMatches;

use settings::Settings;
use settings::DownloadDirCache;

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

pub struct UserParameters {
    pub websites: Vec<Website>,
    pub directory: PathBuf,
    pub search_query: String,
    pub autodownload: bool,
}

pub fn run(args: ArgMatches) {
    let user_parameters = UserParameters::get_params(args);

    let mut torrents: Vec<Torrent> = Vec::new();
    for website in user_parameters.websites {
        torrents.extend(match website {
            Website::Nyaa => {
                nyaa::query(&user_parameters.search_query).unwrap_or_else(|err| {
                    eprintln!("Error requesting data from nyaa: {}", err);
                    process::exit(1);
                })
            },
            Website::Piratebay => { 
                piratebay::query(&user_parameters.search_query).unwrap_or_else(|err| {
                    eprintln!("Error requesting data from nyaa: {}", err);
                    process::exit(1);
                })
            },
        });
    }
    torrents.sort_by_key(|t| Reverse((t.seeders).parse().unwrap_or(0)));

    let magnets = interface::display_torrent_table(&torrents);
    
    if user_parameters.autodownload {
        for m in magnets {
            download_torrent(user_parameters.directory.to_str().unwrap(), &m);
        }
    }
    else {
        for m in magnets {
            println!("{}", m);
        }
    }
}

fn download_torrent(dir: &str, magnet: &str) {
    match process::Command::new("sudo")
        .arg("deluge-console")
        .arg("add")
        .arg("-p")
        .arg(dir)
        .arg(magnet)
        .status() {
            Err(err) => {
                eprintln!("Failed to autodownload using torrent client selected: {}", err);
                println!("{}", magnet);
            },
            Ok(_) => (),
        }
}