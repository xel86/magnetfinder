use comfy_table::{ Table, ContentArrangement };
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

use crate::{ Torrent, UserParameters, Website, Settings };
use std::io;
use std::path::PathBuf;
use std::process;
use clap::ArgMatches;

impl Website {
    fn new(s: &str) -> Result<Website, &'static str> {
        match s.trim().to_lowercase().as_str() {
            "nyaa" => Ok(Website::Nyaa),
            "piratebay" => Ok(Website::Piratebay),
            "all" => Ok(Website::All),
            _ => Err("Unknown website, supported sites: nyaa, piratebay"),
        }
    }
}

enum Media {
    Anime,
    Movie,
    TVShow,
}

impl Media {
    fn new(s: &str) -> Result<Media, &'static str> {
        match s.trim().to_lowercase().as_str() {
            "anime" => Ok(Media::Anime),
            "movie" => Ok(Media::Movie),
            "tvshow" => Ok(Media::TVShow),
            "tv" => Ok(Media::TVShow),
            _ => Err("Unknown media type, supported types: anime, movie, tvshow"),
        }
    }
    
    fn path(&self, settings: &Settings) -> PathBuf {
        match self {
            Media::Anime => settings.anime_dir.clone(),
            Media::Movie => settings.movie_dir.clone(),
            Media::TVShow => settings.tvshow_dir.clone(),
        }
    }
}

impl UserParameters {
    pub fn get_params(args: ArgMatches) -> UserParameters {
        if !args_present(&args) {
            return UserParameters::prompt();
        }
        else {
            return UserParameters::fetch(args);
        }
    }

    // handles user interface for providing user settings instead of cmd arguments
    fn prompt() -> UserParameters {
        let settings = match Settings::fetch() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error reading settings file: {}", e);
                Settings::default()
            },
        };

        UserParameters {
            websites: UserParameters::get_websites(),
            directory: UserParameters::get_media().path(&settings),
            search_query: UserParameters::get_search_query(),
            autodownload: settings.autodownload,
        }
    }

    // parses provided cmd arguments bypassing user interface prompt
    fn fetch(args: ArgMatches) -> UserParameters {
        let mut websites: Vec<Website> = Vec::new();
        if args.is_present("nyaa") { websites.push(Website::Nyaa); }
        if args.is_present("piratebay") { websites.push(Website::Piratebay); }
        if args.is_present("all") { websites.push(Website::All); }
        
        if websites.len() < 1 {
            eprintln!("Must select website to scrape from, -n for nyaa, -p for piratebay, -a for all");
            process::exit(1);
        }

        let directory = match args.value_of("directory") {
            Some(d) => {
                let mut path = PathBuf::from(d);
                if !path.is_dir() {
                    path = Settings::get_downloads_dir()
                }
                path
            }
            None => Settings::get_downloads_dir(),
        };

        UserParameters {
            websites,
            directory,
            search_query: String::from(args.value_of("query").unwrap_or_else(|| {
                eprintln!("Must provide a valid search query (-q/--query \"search term\")");
                process::exit(1);
            })),
            autodownload: args.is_present("download"),
        }
    }

    fn get_websites() -> Vec<Website> {
        loop {
            let mut input = String::new();
            println!("Website(s) to search from? (nyaa, piratebay, all)");

            io::stdin()
                .read_line(&mut input)
                .expect("io error: failed to read website input");

            let websites = match Website::new(&input) {
                Ok(website) => website,
                Err(err) => {
                    println!("{}", err);
                    continue;
                },
            };

            return vec![websites];
        }
    }

    fn get_media() -> Media {
        loop {
            let mut input = String::new();
            println!("Type of media? (anime, movie, tvshow)");

            io::stdin()
                .read_line(&mut input)
                .expect("io error: failed to read media type input");
            
            let type_of_media = match Media::new(&input) {
                Ok(media) => media,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            return type_of_media;
        }
    }

    fn get_search_query() -> String {
        let mut input = String::new();
        println!("Search query: ");

        io::stdin()
            .read_line(&mut input)
            .expect("io error: couldn't read search query input");

        String::from(input.trim())
    }
}

pub fn display_torrent_table(torrents: &[Torrent]) {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["#", "Name", "Size", "Seeds"]);
    
    let table = update_torrent_table(&mut table, torrents);
    println!("{}", table);
}

fn update_torrent_table<'a>(table: &'a mut Table, torrents: &[Torrent]) -> &'a Table {
    for (n, t) in torrents.iter().enumerate() {
        table.add_row(vec![&(n+1).to_string(), &t.title, &t.size, &t.seeders]);
    }

    table
}

pub fn prompt_torrent_selection(torrents: &[Torrent]) -> Vec<&String> {
    loop {
        println!("Select torrent(s) by #:");

        let mut selections = String::new();

        io::stdin()
            .read_line(&mut selections)
            .expect("io error: couldn't read torrent selection input");
        
        let selections: Vec<&str> = selections.trim().split(" ").collect();
        if selections.len() < 1 {
            println!("Please input one or multiple numbers seperated by a space to select torrent(s)");
            continue;
        }

        if selections[0].to_lowercase() == "q" {
            process::exit(1);
        }

        let magnets = match collect_magnet_links(torrents, &selections) {
            Ok(m) => m,
            Err(s) => {
                println!("{}", s);
                continue;
            }
        };
        
        return magnets;
    }
}

fn collect_magnet_links<'a>(torrents: &'a [Torrent], selections: &[&str]) -> Result<Vec<&'a String>, &'static str> {
    let mut magnets = Vec::new();
    for num_str in selections {
        let num: usize = match num_str.parse() {
            Err(_) => {
                return Err("Only input numbers indicated on the left-most column");
            },
            Ok(num) => num,
        };

        if num > torrents.len() || num <= 0 {
            return Err("Input out of range");
        }

        magnets.push(&torrents[num-1].magnet);
    }
    Ok(magnets)
}

fn args_present(args: &ArgMatches) -> bool {
    args.is_present("nyaa") ||
    args.is_present("piratebay") ||
    args.is_present("all") ||
    args.is_present("download") ||
    args.is_present("directory")
}