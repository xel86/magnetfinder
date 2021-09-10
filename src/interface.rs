use std::io;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;
use std::sync::Arc;

use comfy_table::{ Table, ContentArrangement };
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use clap::ArgMatches;

use crate::{ Torrent, UserParameters, Website, Media, Settings};
use crate::settings::DownloadDirCache;

impl Website {
    fn new(s: &str) -> Result<Vec<Website>, &'static str> {
        match s.trim().to_lowercase().as_str() {
            "nyaa" => Ok(vec![Website::Nyaa]),
            "piratebay" => Ok(vec![Website::Piratebay]),
            "all" => Ok(vec![Website::Nyaa, Website::Piratebay]),
            _ => Err("Unknown website, supported sites: nyaa, piratebay"),
        }
    }
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
    
    fn path(&self, settings: &Settings) -> Rc<PathBuf> {
        match self {
            Media::Anime => Rc::clone(&settings.anime_dir),
            Media::Movie => Rc::clone(&settings.movie_dir),
            Media::TVShow => Rc::clone(&settings.tvshow_dir),
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
                eprintln!("error reading settings file: {}\ngenerating default Settings.toml\n", e);
                Settings::generate_settings_file().unwrap_or_else(|err| {
                    eprintln!("error generating new settings file: {}\n", err);
                });
                Settings::default()
            },
        };

        UserParameters {
            websites: UserParameters::get_websites(),
            directory: UserParameters::get_media().path(&settings),
            search_query: UserParameters::get_search_query(),
            search_depth: 1,
            autodownload: settings.autodownload,
        }
    }

    // parses provided cmd arguments bypassing user interface prompt
    fn fetch(args: ArgMatches) -> UserParameters {
        let mut websites: Vec<Website> = Vec::new();
        if args.is_present("nyaa") { websites.push(Website::Nyaa); }
        if args.is_present("piratebay") { websites.push(Website::Piratebay); }
        if args.is_present("all") { websites = Website::new("all").unwrap(); }
        
        if websites.len() < 1 {
            eprintln!("Must select website to scrape from, -n for nyaa, -p for piratebay, -a for all");
            process::exit(1);
        }

        let mut default_directory = DownloadDirCache::new();
        let directory = match args.value_of("directory") {
            Some(d) => {
                let mut path = Rc::new(PathBuf::from(d));
                if !path.is_dir() {
                    path = default_directory.value();
                }
                path
            }
            None => default_directory.value(),
        };

        let search_depth: u32 = match args.value_of("depth") {
            Some(n) => {
                n.trim().parse().unwrap_or(1) 
            }, 
            None => 1,
        };

        UserParameters {
            websites,
            directory,
            search_query: Arc::new(String::from(args.value_of("query").unwrap_or_else(|| {
                eprintln!("Must provide a valid search query (-q/--query \"search term\")");
                process::exit(1);
            }))),
            search_depth,
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

            return websites;
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

    fn get_search_query() -> Arc<String> {
        let mut input = String::new();
        println!("Search query: ");

        io::stdin()
            .read_line(&mut input)
            .expect("io error: couldn't read search query input");

        Arc::new(String::from(input.trim()))
    }
}

pub fn display_torrent_table(torrents: &[Torrent]) -> Vec<&String>{
    let mut torrents_shown: usize = if torrents.len() < 20 { torrents.len() } else { 20 };
    loop {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["#", "Name", "Size", "Seeds"]);
    
        let table = update_torrent_table(&mut table, &torrents[0..torrents_shown]);
        println!("{}", table);

        match prompt_torrent_selection(&torrents) {
            Some(m) => return m,
            None => (),
        };

        torrents_shown =
        if torrents.len() < torrents_shown+20 { torrents.len() } else { torrents_shown+20 };
    }
}

fn update_torrent_table<'a>(table: &'a mut Table, torrents: &[Torrent]) -> &'a Table {
    for (n, t) in torrents.iter().enumerate() {
        table.add_row(vec![&(n+1).to_string(), &t.title, &t.size, &t.seeders]);
    }

    table
}

pub fn prompt_torrent_selection(torrents: &[Torrent]) -> Option<Vec<&String>> {
    loop {
        println!("Type 'n' to display 20 more torrents, or select torrent(s) by #:");

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

        if selections[0].to_lowercase() == "n" {
            return None;
        }

        let magnets = match collect_magnet_links(torrents, &selections) {
            Ok(m) => m,
            Err(s) => {
                println!("{}", s);
                continue;
            }
        };
        
        return Some(magnets);
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
