mod nyaa;

use std::process;

#[derive(Debug)]
pub struct Torrent {
    pub title: String,
    pub magnet: String,
    pub size: String,
    pub seeders: String,
}

pub fn run() {
    let v1 = nyaa::query("boku").unwrap_or_else(|err| {
        eprintln!("Error requesting data from nyaa: {}", err);
        process::exit(1);
    });

    for t in v1 {
        println!("{} - {} - {} - \n{}", t.title, t.size, t.seeders, t.magnet);
    }
}
