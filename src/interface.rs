use std::io;
use std::path::Path;

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
    
    fn path(&self) -> &Path {
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