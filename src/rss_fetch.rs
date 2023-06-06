use crate::config::AppConfig;
use crate::db::Database;
use crate::http::post_article;
use crate::model::ArticleInfo;
use crate::telegram::send_message;

use reqwest::Client;
use rss::Channel;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::time;

pub async fn process_rss_feeds(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = Database::new("rss_cache.db")?;
    let client = Client::new();

    let input_file = File::open("rss_feeds.txt")?;
    let buffered = io::BufReader::new(input_file);

    for line in buffered.lines() {
        let url = line?;

        loop {
            let content = client.get(&url).send().await?.text().await?;
            let channel = content.parse::<Channel>()?;

            for item in channel.items() {
                let title = item.title().unwrap_or_default().to_owned();
                let description = item.description().unwrap_or_default().to_owned();
                let link = item.link().unwrap_or_default().to_owned();

                let article = ArticleInfo {
                    title: title.clone(),
                    description: description.clone(),
                    link: link.clone(),
                };

                if cache.insert_item(&article).is_ok() {
                    match config.mode.as_str() {
                        "test" => {
                            println!("{:#?}", article);
                        }
                        "telegram" => {
                            send_message(
                                &client,
                                &config.bot_token.as_ref().unwrap(),
                                &config.chat_id.as_ref().unwrap(),
                                &title,
                                &description,
                                &link,
                            )
                            .await?;
                            std::process::exit(0);
                        }
                        "http" => {
                            post_article(&client, &article).await?;
                        }
                        _ => unreachable!(),
                    }
                } else {
                    println!("Article already seen: {}", title);
                }
            }

            time::sleep(Duration::from_secs(10)).await;
        }
    }

    Ok(())
}
