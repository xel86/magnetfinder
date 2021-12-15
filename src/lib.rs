pub mod interface;
pub mod nyaa;
pub mod piratebay;
pub mod settings;
pub mod types;
pub mod yts;

use std::cmp::Reverse;
use std::process;
use std::sync::{mpsc, Arc};

use clap::ArgMatches;
use ureq::{Agent, AgentBuilder};

use types::{Media, Settings, Sort, Torrent, TorrentClient, UserParameters, Website};

pub fn run(args: ArgMatches) {
    let user_parameters = UserParameters::get_params(args);

    let client = Arc::new(match build_http_client(&user_parameters.proxy) {
        Ok(client) => client,
        Err(_) => Agent::new(),
    });

    let (tx, rx) = mpsc::channel();
    for website in user_parameters.websites {
        match website {
            Website::Nyaa => nyaa::query(
                &client,
                tx.clone(),
                &user_parameters.search_query,
                user_parameters.search_depth,
            ),
            Website::Piratebay => piratebay::query(
                &client,
                tx.clone(),
                &user_parameters.search_query,
                user_parameters.search_depth,
            ),
            Website::YTS => yts::query(
                &client,
                tx.clone(),
                &user_parameters.search_query,
                user_parameters.search_depth,
            ),
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

    if torrents.is_empty() {
        eprintln!("No torrents found matching search query");
        return;
    }

    if torrents.len() > user_parameters.num_torrents_shown {
        torrents.truncate(user_parameters.num_torrents_shown);
    }

    if !user_parameters.no_interactive {
        let magnets = interface::display_torrent_table(&torrents);

        if user_parameters.autodownload {
            for m in magnets {
                download_torrent(
                    &user_parameters.torrent_client,
                    user_parameters.directory.to_str().unwrap(),
                    m,
                );
            }
        } else {
            for m in magnets {
                println!("{}", m);
            }
        }
    } else {
        for torrent in &torrents {
            println!("{}\t{}", torrent.title, torrent.magnet);
        }
    }
}

fn build_http_client(proxy: &str) -> Result<Agent, ureq::Error> {
    if proxy.is_empty() {
        Ok(Agent::new())
    } else {
        Ok(AgentBuilder::new().proxy(ureq::Proxy::new(proxy)?).build())
    }
}

fn download_torrent(client: &TorrentClient, dir: &str, magnet: &str) {
    match client {
        TorrentClient::Deluge => call_deluge(dir, magnet),
        TorrentClient::Transmission => call_transmission(dir, magnet),
        TorrentClient::QBittorrent => call_qbittorrent(dir, magnet),
        TorrentClient::Unknown => {
            eprintln!("Unknown or empty torrent client in config file. Edit config with supported torrent client to used autodownload");
            println!("{}", magnet);
        }
    }
}

fn call_deluge(dir: &str, magnet: &str) {
    if let Err(err) = process::Command::new("sudo")
        .arg("deluge-console")
        .arg("add")
        .arg("--path")
        .arg(dir)
        .arg(magnet)
        .status()
    {
        eprintln!(
            "Failed to autodownload using torrent client deluge: {}",
            err
        );
        println!("{}", magnet);
    }
}

fn call_transmission(dir: &str, magnet: &str) {
    if let Err(err) = process::Command::new("transmission-remote")
        .arg("-w")
        .arg(dir)
        .arg("-a")
        .arg(magnet)
        .status()
    {
        eprintln!(
            "Failed to autodownload using torrent client transmission: {}",
            err
        );
        println!("{}", magnet);
    }
}

fn call_qbittorrent(dir: &str, magnet: &str) {
    if let Err(err) = process::Command::new("qbt")
        .arg("add")
        .arg("--save-path")
        .arg(dir)
        .arg("--magnet")
        .arg(magnet)
        .status()
    {
        eprintln!(
            "Failed to autodownload using torrent client qbittorrent (qbt): {}",
            err
        );
        println!("{}", magnet);
    }
}
