extern crate reqwest;
extern crate scraper;

use crate::Torrent;
use scraper::{Html, Selector, element_ref::ElementRef};


pub fn query(query: &str) -> Result<Vec<Torrent>, reqwest::Error> {
    let mut results = Vec::new();
    let url = format!("https://nyaa.si/?f=0&c=0_0&q={}{}", query, "&s=seeders&o=desc");

    let body = reqwest::blocking::get(&url)?.text()?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse("tbody tr").unwrap();

    for table_row in document.select(&selector) {
        let title = match get_title(query, &table_row) {
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

fn get_title(query: &str, table_row: &ElementRef) -> Option<String> { 
    let selector = Selector::parse("td[colspan] a").unwrap();

    for data in table_row.select(&selector) {
        let title = match data.value().attr("title") {
            Some(t) => t,
            None => continue,
        };

        if title.to_lowercase().contains(query) {
            return Some(String::from(title));
        }
    }

    None
}

fn get_magnet(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse("td.text-center a").unwrap();

    for data in table_row.select(&selector) {
        let magnet = match data.value().attr("href") {
            Some(m) => m,
            None => continue,
        };

        if magnet.contains("magnet") {
            return Some(String::from(magnet));
        }
    }

    None
}

fn get_size(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse("td.text-center").unwrap();

    let size = match table_row.select(&selector).nth(1) {
        Some(s) => s.inner_html(),
        None => return None,
    };

    Some(size)
}

fn get_seeders(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse("td.text-center").unwrap();

    let seeders = match table_row.select(&selector).nth(3) {
        Some(s) => s.inner_html(),
        None => return None,
    };

    Some(seeders)
}
