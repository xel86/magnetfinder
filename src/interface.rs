use comfy_table::{ Table, ContentArrangement };
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

use crate::Torrent;
use std::io;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub enum Website {
    Nyaa,
    Piratebay,
    All,
}

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

pub enum Media {
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
    
    fn _path(&self) -> &Path {
        match self {
            Media::Anime => Path::new("./Downloads/"),
            Media::Movie => Path::new("./Downloads/"),
            Media::TVShow => Path::new("./Downloads/"),
        }
    }
}

pub struct UserParameters {
    pub website: Website,
    pub type_of_media: Media,
    pub search_query: String,
}

impl UserParameters {
    pub fn prompt() -> UserParameters {
        UserParameters {
            website: UserParameters::get_websites(),
            type_of_media: UserParameters::get_media(),
            search_query: UserParameters::get_search_query(),
        }
    }

    fn get_websites() -> Website {
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

pub fn prompt_magnet_selection(torrents: &[Torrent]) -> Vec<&String> {
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