pub mod config;
mod db;
mod http;
mod model;
mod telegram;

use crate::config::{AppConfig, Mode};
use crate::db::Database;
use crate::http::post_article;
use crate::model::ArticleInfo;
use crate::telegram::send_message;

use rss::Channel;
use std::fs::File;
use std::io::{self, Read};
use std::time::Duration;
use tokio::time;

pub async fn process_rss_feeds(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = Database::new("rss_cache.db")?;
    let input_file = File::open("rss_feeds.txt")?;
    let mut contents = String::new();
    io::BufReader::new(input_file).read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    // In the future, add websub support here
    // take in two params, IP address (no default) and port (default 80)
    // for each websub enabled feed, call a module that will handle subscribing to the feed
    // need to also abstract the data processing steps so that they can be called from here
    // or the websub module
    // Also need to store websub subscriptions or at least ensure that startup
    // behavior is fine if a sub is already leased

    loop {
        for url in &lines {
            let content = match reqwest::get(url).await {
                Ok(resp) => {
                    println!("Fetched {}", url);
                    resp.bytes().await.unwrap()
                }
                Err(e) => {
                    eprintln!("Error fetching {}: {}", url, e);
                    continue;
                }
            };
            let channel = match Channel::read_from(&*content) {
                Ok(channel) => channel,
                Err(e) => {
                    eprintln!("Error parsing {}: {}", url, e);
                    continue;
                }
            };

            for item in channel.items() {
                let title = item.title().unwrap_or_default().to_owned();
                let description = item
                    .description()
                    .map(|d| d.to_owned())
                    .filter(|s| !s.is_empty());
                let link = item.link().unwrap_or_default().to_owned();

                let article = ArticleInfo {
                    title: title.clone(),
                    description: description.clone(),
                    link: link.clone(),
                };

                if cache.insert_item(&article).is_ok() {
                    match config.mode {
                        Mode::Test => {
                            println!("\n{:#?}", article);
                        }
                        Mode::Telegram => {
                            send_message(
                                &config.bot_token.as_ref().unwrap(),
                                &config.chat_id.as_ref().unwrap(),
                                &article,
                            )
                            .await?;
                            return Ok(());
                        }
                        Mode::Http => {
                            post_article(&article).await?;
                        }
                    }
                    if config.one_at_a_time {
                        return Ok(());
                    }
                } else {
                    println!("Skipping: {}", title);
                }
            }
            time::sleep(Duration::from_millis(config.poll_interval as u64)).await;
        }
    }
}
