use crate::config::AppConfig;
use crate::db::Database;
use crate::model::Article;
use crate::push::push_data;

use reqwest::Client;
use rss::Channel;
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::{task, time};

pub async fn process_rss_feeds(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Client::new());
    let config = Arc::new(config);
    let cache = Arc::new(Database::new("rss_cache.db")?);
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

    let mut watcher_handles: Vec<JoinHandle<()>> = vec![];

    for url in &lines {
        let client = client.clone();
        let url = url.clone();
        let cache = cache.clone();
        let config = config.clone();

        let handle = task::spawn(async move {
            println!("Running thread for {}", url);
            loop {
                let content = match client
                    .get(&url)
                    .header(reqwest::header::USER_AGENT, "curl/7.88.1")
                    .send()
                    .await
                {
                    Ok(resp) => {
                        println!("Fetched {}", url);
                        resp.bytes().await.unwrap()
                    }
                    Err(e) => {
                        eprintln!("Error fetching {}: {}", url, e);
                        time::sleep(Duration::from_millis(config.poll_interval)).await;
                        continue;
                    }
                };
                let channel = match Channel::read_from(&*content) {
                    Ok(channel) => channel,
                    Err(e) => {
                        eprintln!("Error parsing {}: {}", url, e);
                        time::sleep(Duration::from_millis(config.poll_interval)).await;
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

                    let article = Article {
                        title: title.clone(),
                        description: description.clone(),
                        link: link.clone(),
                    };

                    match cache.insert_item(&article) {
                        Ok(_) => {
                            push_data(&client, &config, &article).await;

                            if config.one_at_a_time {
                                std::process::exit(0);
                            }
                        }
                        Err(rusqlite::Error::SqliteFailure(error, _))
                            if error.code == rusqlite::ErrorCode::ConstraintViolation => {}
                        Err(e) => {
                            eprintln!("Error inserting {}: {}", title, e);
                        }
                    }
                }
                time::sleep(Duration::from_millis(config.poll_interval)).await;
            }
        });
        watcher_handles.push(handle);
    }
    for handle in watcher_handles {
        handle.await?;
    }
    Ok(())
}
