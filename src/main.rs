use blazing_fast_rss_watcher::{config::AppConfig, process_rss_feeds};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env_args()?;

    process_rss_feeds(config).await?;

    Ok(())
}
