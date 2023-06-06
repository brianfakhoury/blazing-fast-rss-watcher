mod db;
mod model;
mod server;

use db::Database;
use model::MyItem;
use server::start_server_if_test;

use reqwest::Client;
use rss::Channel;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::new("rss_cache.db")?;
    let client = Client::new();

    start_server_if_test();

    let input_file = File::open("rss_feeds.txt")?;
    let buffered = io::BufReader::new(input_file);

    time::sleep(Duration::from_secs(3)).await;

    for line in buffered.lines() {
        let url = line?;

        loop {
            let content = client.get(&url).send().await?.text().await?;
            let channel = content.parse::<Channel>()?;

            for item in channel.items() {
                let title = item.title().unwrap_or_default().to_owned();
                let link = item.link().unwrap_or_default().to_owned();

                let my_item = MyItem {
                    title: title.clone(),
                    link: link.clone(),
                };

                if db.insert_item(&my_item).is_ok() {
                    client
                        .post("http://localhost:3030/endpoint")
                        .json(&my_item)
                        .send()
                        .await?;
                }
            }

            time::sleep(Duration::from_secs(10)).await;
        }
    }

    Ok(())
}
