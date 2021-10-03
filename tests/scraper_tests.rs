use magnetfinder::nyaa;
use magnetfinder::piratebay;
use magnetfinder::yts;

use ureq::Agent;

#[test]
fn nyaa_produces_results() {
    let results = nyaa::fetch_page_results(&Agent::new(), "episode", 1).unwrap();

    assert!(!results.is_empty(), "returned torrent vector was empty");

    assert!(
        results[0].title.to_lowercase().contains("episode"),
        "returned torrent's title didn't contain search query"
    );
    assert!(
        results[0].magnet.contains("magnet"),
        "returned magnet link is incorrect"
    );
    assert!(
        !results[0].size.is_empty(),
        "returned torrent's size was empty"
    );
    assert!(
        !results[0].seeders.is_empty(),
        "returned torrent's seeders was empty"
    );
}

#[test]
fn piratebay_produces_results() {
    let results = piratebay::fetch_page_results(&Agent::new(), "episode", 1).unwrap();

    assert!(!results.is_empty(), "returned torrent vector was empty");

    assert!(
        results[0].title.to_lowercase().contains("episode"),
        "returned torrent's title didn't contain search query"
    );
    assert!(
        results[0].magnet.contains("magnet"),
        "returned magnet link is incorrect"
    );
    assert!(
        !results[0].size.is_empty(),
        "returned torrent's size was empty"
    );
    assert!(
        !results[0].seeders.is_empty(),
        "returned torrent's seeders was empty"
    );
}

#[test]
fn yts_produces_results() {
    let results = yts::fetch_page_results(&Agent::new(), "star", 1).unwrap();

    assert!(!results.is_empty(), "returned torrent vector was empty");

    assert!(
        results[0].title.to_lowercase().contains("star"),
        "returned torrent's title didn't contain search query"
    );
    assert!(
        results[0].magnet.contains("magnet"),
        "returned magnet link is incorrect"
    );
    assert!(
        !results[0].size.is_empty(),
        "returned torrent's size was empty"
    );
    assert!(
        !results[0].seeders.is_empty(),
        "returned torrent's seeders was empty"
    );
}
