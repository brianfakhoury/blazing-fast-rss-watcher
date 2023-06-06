mod config;
mod db;
mod http;
mod model;
mod rss_fetch;
mod telegram;

use config::AppConfig;
use rss_fetch::process_rss_feeds;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env_args()?;

    process_rss_feeds(&config).await?;

    Ok(())
}
