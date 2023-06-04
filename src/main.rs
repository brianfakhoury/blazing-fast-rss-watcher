use rss::Channel;
use reqwest::Client;
use rusqlite::{params, Connection, Result};
use std::time::Duration;
use tokio::time;
use warp::Filter;
use serde_derive::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Serialize, Debug, Deserialize)] 
struct MyItem {
    title: String,
    link: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("rss_cache.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
             title TEXT NOT NULL,
             link TEXT NOT NULL UNIQUE
         )",
        [],
    )?;

    let client = Client::new();

    // Launch dummy server if "test" flag is set
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        let route = warp::path("endpoint")
            .and(warp::post())
            .and(warp::body::json())
            .map(|item: MyItem| {
                println!("{:#?}", item);
                warp::reply()
            });
        println!("Launching test server at http://localhost:3030/endpoint");
        tokio::spawn(warp::serve(route).run(([127, 0, 0, 1], 3030)));
    }

    // Open the text file
    let input_file = File::open("rss_feeds.txt")?;
    let buffered = io::BufReader::new(input_file);

    // Add a delay here
    time::sleep(Duration::from_secs(3)).await;

    // Read the file line by line
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

                match conn.execute(
                    "INSERT INTO items (title, link) VALUES (?, ?)",
                    params![title, link],
                ) {
                    Ok(_) => {
                        client.post("http://localhost:3030/endpoint")
                            .json(&my_item)
                            .send()
                            .await?;
                    }
                    Err(rusqlite::Error::SqliteFailure(error, _)) if error.code == rusqlite::ErrorCode::ConstraintViolation => {}
                    Err(err) => return Err(err.into()),
                }
            }

            time::sleep(Duration::from_secs(10)).await;
        }
    }

    Ok(())
}
