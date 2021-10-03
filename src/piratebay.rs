use std::sync::{Arc, mpsc::Sender};
use std::thread;

use scraper::{Html, Selector, element_ref::ElementRef};
use ureq::Agent;

use crate::Torrent;

pub fn query(client: &Arc<Agent>, tx: Sender<Vec<Torrent>>, query: &Arc<String>, depth: u32) {
    for page in 1..=depth {
        let t_tx = Sender::clone(&tx);
        let t_client = Arc::clone(&client);
        let t_query = Arc::clone(&query);

        thread::spawn(move || {
            let torrents = fetch_page_results(&t_client, &t_query, page).unwrap_or_else(|err| {
                eprintln!("Error requesting data from piratebay: {}", err);
                vec![]
            });

            t_tx.send(torrents).unwrap(); 
        });
    }
}

pub fn fetch_page_results(client: &Agent, query: &str, page_number: u32) -> Result<Vec<Torrent>, ureq::Error> {
    let mut results = Vec::new();

    let formatted_query = query.replace(" ", "%20");
    let url = format!("https://www.tpb.party/search/{}/{}/99/0", formatted_query, page_number);
    let body = client.get(&url).call()?.into_string()?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("tbody tr").unwrap();

    for table_row in document.select(&selector) {
        let title = match get_title(&table_row) {
            Some(title) => title,
            None => continue,
        };
        let magnet = match get_magnet(&table_row) {
            Some(magnet) => magnet,
            None => continue,
        };
        let size = match get_size(&table_row) {
            Some(size) => size,
            None => continue,
        };
        let seeders = match get_seeders(&table_row) {
            Some(seeders) => seeders,
            None => continue,
        };

        results.push(Torrent { 
            title, 
            magnet,
            size,
            seeders,
        });
    }

    Ok(results)
}

fn get_title(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse(".detLink").unwrap();

    let title = match table_row.select(&selector).next() {
        Some(t) => t.inner_html(),
        None => return None,
    };

    if title.is_empty() { return None }

    Some(title)
}

fn get_magnet(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse("[alt='Magnet link']").unwrap();

    let magnet = match table_row.select(&selector).next() {
        Some(p) => { match p.parent() {
            Some(parent) => { match parent.value().as_element().unwrap().attr("href") {
                Some(m) => m,
                None => return None,
            }},
            None => return None,
        }},
        None => return None,
    };

    if magnet.contains("magnet") {
        let cleaned_magnet = magnet.split("&tr=").next().unwrap_or(magnet);
        return Some(String::from(cleaned_magnet));
    }

    None
}

fn get_size(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse(".detDesc").unwrap();

    let desc = match table_row.select(&selector).next() {
        Some(s) => s.inner_html(),
        None => return None,
    };

    let split: Vec<&str> = desc.split(", ").collect();
    let size = match split.get(1) {
        Some(s) => {
            s.replace("Size ", "").replace("&nbsp;", " ")
        },
        None => return None,
    };

    Some(size)
}

fn get_seeders(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse("td").unwrap();

    let seeders = match table_row.select(&selector).nth(2) {
        Some(s) => s.inner_html(),
        None => return None,
    };

    Some(seeders)
}
