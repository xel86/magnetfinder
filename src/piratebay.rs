use scraper::{Html, Selector, element_ref::ElementRef};

use crate::Torrent;

pub fn query(query: &str, depth: u32) -> Result<Vec<Torrent>, reqwest::Error> {
    let mut results = Vec::new();

    for page_number in 1..=depth {
    let formatted_query = query.replace(" ", "%20");
    let url = format!("https://www.tpb.party/search/{}/{}/99/0", formatted_query, page_number);
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
    let selector = Selector::parse(".detLink").unwrap();

    let title = match table_row.select(&selector).next() {
        Some(t) => t.inner_html(),
        None => return None,
    };

    if title == "" { return None }

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
