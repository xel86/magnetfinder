pub mod nyaa;
pub mod piratebay;
pub mod yts;
pub mod interface;
pub mod types;
pub mod settings;

use std::process;
use std::cmp::Reverse;
use std::sync::{mpsc, Arc};

use clap::ArgMatches;
use reqwest::{blocking::Client};

use types::{Settings, UserParameters, Website, Media, Sort, Torrent};

pub fn run(args: ArgMatches) {
    let user_parameters = UserParameters::get_params(args);

    let client = Arc::new(match build_reqwest_client(&user_parameters.proxy) {
        Ok(client) => client,
        Err(_) => Client::new(),
    });

    let (tx, rx) = mpsc::channel();
    for website in user_parameters.websites {
        match website {
            Website::Nyaa => {
                nyaa::query(&client, tx.clone(), &user_parameters.search_query, user_parameters.search_depth)
            },
            Website::Piratebay => { 
                piratebay::query(&client, tx.clone(), &user_parameters.search_query, user_parameters.search_depth)
            },
            Website::YTS => {
                yts::query(&client, tx.clone(), &user_parameters.search_query, user_parameters.search_depth)
            }
        };
    }
    drop(tx);

    let mut torrents: Vec<Torrent> = Vec::new();
    for received_torrents in rx {
        torrents.extend(received_torrents);
    }

    match user_parameters.sort_preference {
        Sort::Size => torrents.sort_by_key(|t| Reverse(t.get_size_as_i64())),
        Sort::Seeds => torrents.sort_by_key(|t| Reverse((t.seeders).parse().unwrap_or(0))),
    }

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

fn build_reqwest_client(proxy: &str) -> Result<Client, reqwest::Error> {
    if proxy.is_empty() {
        Client::builder()
            .build()
    }
    else {
        Client::builder()
            .proxy(reqwest::Proxy::all(proxy)?)
            .build()
    }
}

fn download_torrent(dir: &str, magnet: &str) {
    if let Err(err) = process::Command::new("sudo")
        .arg("deluge-console")
        .arg("add")
        .arg("--path")
        .arg(dir)
        .arg(magnet)
        .status() {
            eprintln!("Failed to autodownload using torrent client selected: {}", err);
            println!("{}", magnet);
        }
}
