mod nyaa;
mod interface;
mod settings;

use std::process;
use std::path::PathBuf;
use clap::ArgMatches;

use settings::Settings;

pub struct Torrent {
    pub title: String,
    pub magnet: String,
    pub size: String,
    pub seeders: String,
}

pub enum Website {
    Nyaa,
    Piratebay,
    All,
}

pub struct UserParameters {
    pub websites: Vec<Website>,
    pub directory: PathBuf,
    pub search_query: String,
    pub autodownload: bool,
}

pub fn run(args: ArgMatches) {
    let settings = match Settings::fetch() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error reading settings file: {}", e);
            Settings::default()
        },
    };

    let user_parameters = UserParameters::get_params(args, &settings);

    let mut torrents: Vec<Torrent> = Vec::new();
    for website in user_parameters.websites {
        torrents.extend(match website {
            Website::Nyaa => {
                nyaa::query(&user_parameters.search_query).unwrap_or_else(|err| {
                    eprintln!("Error requesting data from nyaa: {}", err);
                    process::exit(1);
                })
            },
            Website::Piratebay => process::exit(1),
            Website::All => process::exit(1),
        });
    }

    interface::display_torrent_table(&torrents);

    let magnets = interface::prompt_torrent_selection(&torrents);
    
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