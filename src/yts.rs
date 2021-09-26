use std::sync::{Arc, mpsc::Sender};
use std::thread;

use serde::Deserialize;
use reqwest::{blocking::Client};

use crate::Torrent;

#[allow(dead_code)]
#[derive(Deserialize)]
struct YTSTorrent {
    url: String,
    hash: String,
    quality: String,
    r#type: String,
    seeds: usize,
    peers: usize,
    size: String,
    size_bytes: usize,
    date_uploaded: String,
    date_uploaded_unix: i64
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct YTSMovie {
    id: u32,
    url: String,
    imdb_code: String,
    title: String,
    title_english: String,
    title_long: String,
    slug: String,
    year: i32,
    runtime: u32,
    genres: Vec<String>,
    summary: String,
    yt_trailer_code: String,
    language: String,
    mpa_rating: String,
    background_image: String,
    background_image_original: String,
    small_cover_image: String,
    medium_cover_image: String,
    large_cover_image: String,
    state: String,
    torrents: Vec<YTSTorrent>
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct YTSData {
    movie_count: usize,
    limit: usize,
    page_number: usize,
    movies: Vec<YTSMovie>
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct YTSResponse {
    status: String,
    status_message: String,
    data: YTSData
}

pub fn query(client: &Arc<Client>, tx: Sender<Vec<Torrent>>, query: &Arc<String>, depth: u32) {
    for page in 1..=depth {
        let t_tx = Sender::clone(&tx);
        let t_client = Arc::clone(&client);
        let t_query = Arc::clone(&query);

        thread::spawn(move || {
            let torrents = fetch_page_results(&t_client, &t_query, page).unwrap_or_else(|err| {
                // json decode errors will occur when search_depth is greater than
                // the number of pages yts has for a search, ignore error messages for this
                if !err.to_string().contains("decoding") {
                    eprintln!("Error requesting data from yts: {:?}", err);
                }

                vec![]
            });

            t_tx.send(torrents).unwrap(); 
        });
    }
}

pub fn fetch_page_results(client: &Client, query: &str, page_number: u32) -> Result<Vec<Torrent>, reqwest::Error> {
    let mut results = Vec::new();

    let formatted_query = query.replace(" ", "+");
    let url = format!(
        "https://yts.mx/api/v2/list_movies.json?query_term={}&page={}",
        formatted_query,
        page_number
    );
    let body = client.get(&url).send()?.json::<YTSResponse>()?;

    for movie in body.data.movies {
        let title = movie.title_long;
        let slug = movie.slug;

        for torrent in movie.torrents {
            results.push(Torrent { 
                title: title.clone(), 
                magnet: make_magnet(torrent.hash, slug.clone()),
                size: torrent.size,
                seeders: torrent.seeds.to_string(),
            });
        }
    }

    Ok(results)
}

fn make_magnet(info_hash: String, name: String) -> String {
    // recommended trackers from YTS API docs (https://yts.mx/api)
    let trackers = [
        "udp://open.demonii.com:1337/announce",
        "udp://tracker.openbittorrent.com:80",
        "udp://tracker.coppersurfer.tk:6969",
        "udp://glotorrents.pw:6969/announce",
        "udp://tracker.opentrackr.org:1337/announce",
        "udp://torrent.gresille.org:80/announce",
        "udp://p4p.arenabg.com:1337",
        "udp://tracker.leechers-paradise.org:6969"
    ].join("&tr=");

    format!("magnet:?xt=urn:btih:{}&dn={}&tr={}", info_hash, name, trackers)
}
