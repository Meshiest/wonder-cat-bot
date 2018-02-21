extern crate regex;
extern crate reqwest;
extern crate scraper;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use regex::Regex;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Deserialize)]
struct Config {
    feed: Feed,
    telegram: Telegram,
}

#[derive(Deserialize)]
struct Feed {
    url: String,
    regex: String,
    tracker_file: String,
}

#[derive(Deserialize)]
struct Telegram {
    token: String,
    chat_id: i64,
}

// Reads the config file, returns config as a struct
fn read_config() -> Config {
    let mut f = File::open("Config.toml").expect("Error opening Config.toml");
    // Read the file into a vecotr
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("Error reading Config.toml");

    // Parse the vector as utf8 and parse it as a toml config
    return toml::from_str(String::from_utf8(buf).unwrap().as_ref())
        .expect("Error parsing Config.toml");
}

// Writes a type to a file
fn write_tracker<T: std::string::ToString>(path: &str, index: T) {
    let f = &mut File::create(path).expect(format!("Error creating {}", path).as_ref());
    f.write(index.to_string().as_bytes().as_ref())
        .expect(format!("Error writing to {}", path).as_ref());
    f.sync_all().expect("Error syncing to disk");
}

// Reads a type from a file
fn read_tracker(path: &str) -> i32 {
    if let Ok(mut f) = File::open(path) {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .expect(format!("Error reading {}", path).as_ref());
        if let Ok(res) = String::from_utf8(buf).unwrap().parse::<i32>() {
            res
        } else {
            -1
        }
    } else {
        -1
    }
}

fn open_url(url: &str) -> Result<Html, reqwest::Error> {
    Ok(Html::parse_document(get_req(url)?.as_ref()))
}

// Runs a get request
fn get_req(uri: &str) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(uri)?.text()?)
}

fn main() {
    // Load stuff from config
    let conf: Config = read_config();
    let telegram = conf.telegram;
    let tracker_file = conf.feed.tracker_file;
    let index_regex = Regex::new(conf.feed.regex.as_ref()).expect("Invalid feed regex");

    // Helper function for sending telegram messages
    let send = |msg| {
        get_req(
            format!(
                "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                telegram.token, telegram.chat_id, msg
            ).as_ref(),
        )
    };

    // Loop our check
    loop {
        // Determine what our last comic was
        let last_comic = read_tracker(tracker_file.as_ref());

        if let Ok(feed_dom) = open_url(conf.feed.url.as_ref()) {
            // Select
            feed_dom
                .select(&Selector::parse(".element .title a").unwrap())
                .map(|elem| elem.value().attr("href").unwrap().to_string())
                .collect::<Vec<String>>()
                .iter()
                .rev()
                // Create a tuple with our page url and the comic index
                .map(|url| { 
                    (
                        url.clone(),
                        index_regex
                            .captures(url.as_ref())
                            .and_then(|cap| Some(cap[1].parse::<i32>().unwrap())),
                    )
                })
                // Select only new comics
                .filter(|pair| pair.1.is_some() && pair.1.unwrap() > last_comic)
                .for_each(|pair| {
                    let url = pair.0;
                    let i = pair.1.unwrap();

                    // Open one of the comic links
                    if let Ok(page_dom) = open_url(url.as_ref()) {

                        // Get the image url from the page
                        let image_url = page_dom
                            .select(&Selector::parse("img.open").unwrap())
                            .next()
                            .and_then(|n| n.value().attr("src"))
                            .and_then(|s| Some(s.to_string()));

                        if image_url.is_some() && send(image_url.unwrap()).is_ok() {
                            println!("New Comic #{}!", i);
                            // Update our tracker file
                            write_tracker(tracker_file.as_ref(), i);
                            // Wait 5 seconds so we do not spam requests
                            sleep(Duration::from_secs(5));
                        }
                    }
                });
            // Wait an hour until updating again!
            sleep(Duration::from_secs(60 * 60));
        } else {
            // For some reason the website is down, so we should retry in 15 minutes
            println!("Website may be down, retrying in 15 minutes");
            sleep(Duration::from_secs(15 * 60));
        }
    }
}
