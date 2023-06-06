use crate::model::ArticleInfo;
use reqwest::Client;
use std::error::Error;

pub async fn post_article(client: &Client, article: &ArticleInfo) -> Result<(), Box<dyn Error>> {
    client
        .post("http://localhost:3030/endpoint")
        .json(&article)
        .send()
        .await?;

    Ok(())
}
