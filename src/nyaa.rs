use scraper::{Html, Selector, element_ref::ElementRef};

use crate::Torrent;

pub fn query(query: &str, depth: u32) -> Result<Vec<Torrent>, reqwest::Error> {
    let mut results = Vec::new();

    for page_number in 1..=depth {
    let formatted_query = query.replace(" ", "+");
    let url = format!("https://nyaa.si/?f=0&c=0_0&q={}&s=seeders&o=desc&p={}", formatted_query, page_number);
    let body = reqwest::blocking::get(&url)?.text()?;

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
    }

    Ok(results)
}

fn get_title(table_row: &ElementRef) -> Option<String> {
    let selector = Selector::parse("td[colspan] a").unwrap();

    let title = match table_row.select(&selector).nth(1) {
        Some(t) => t.value().attr("title").unwrap_or(""),
        None => return None,
    };

    if title == "" { return None }

    Some(String::from(title))
}

fn _get_title_verbose(query: &str, table_row: &ElementRef) -> Option<String> { 
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
            let cleaned_magnet = magnet.split("&tr=").next().unwrap_or(magnet);
            return Some(String::from(cleaned_magnet));
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
