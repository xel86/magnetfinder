# magnetfinder
Multi-threaded CLI torrent aggregator; scrapes torrent results from multiple websites and delivers them into a table in your terminal!

Supported Websites:
- nyaa 
- piratebay
- YTS

Supported torrent client for autodownloads:
- deluge-console

![](https://i.imgur.com/piuGz7w.png)

## Usage

Running magnetfinder without any arguments will launch interactive mode, prompting for similar information set by flags.

#### Flags/Arguments<br>
```-q, --query <query>``` search query to use<br>
```-n, --nyaa``` scrape nyaa for torrents<br>
```-p, --piratebay``` scrape piratebay for torrents<br>
```-y, --yts``` get torrents from YIFY/YTS<br>
```-a, --all``` scrape all available websites together<br>
```-d, --download``` autodownload the torrent(s) selected<br>
```--depth <depth>```  specifies how many pages to search through for each website, default is 1<br>
```--dir <directory>``` directory to download torrent if autodownload was toggled<br>
```--sort <seeds/size>``` allows you to specifiy if the torrent table is sorted by seeders or size<br>
```--proxy <proxy url>``` allows you to set a proxy to use when making web requests to torrent websites & api<br>
  
#### Configuration

Settings.toml (for setting default behavior, such as download directories & autodownload) is located in an OS specific directory:<br>
```~/.config/magnetfinder/``` on Linux<br>
```/AppData/Roaming/magnetfinder``` on Windows<br>
```/Library/Application Support/magnetfinder/``` on macOS<br>
  

## Installation
First install rust if you haven't already: https://www.rust-lang.org/tools/install<br>

From Cargo/Crate: ```cargo install magnetfinder```<br>

From Source: 
- ```git clone https://github.com/bleusakura/magnetfinder.git``` then ```cargo build --release```
- After building, the binary will be located in ```./target/release/```, which can then be moved elsewhere.

You can also decide to skip compiling and download a binary from the [releases section](https://github.com/bleusakura/magnetfinder/releases)
