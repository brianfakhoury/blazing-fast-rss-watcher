use crate::config::{AppConfig, Mode};
use crate::model::Article;
use crate::telegram::send_message;
use reqwest::Client;

pub async fn push_data(client: &Client, config: &AppConfig, article: &Article) {
    match config.mode {
        Mode::Test => {
            println!("\n{:#?}", article);
        }
        Mode::Telegram => {
            println!("Sending message... {:?}", article);
            match send_message(
                client,
                config.bot_token.as_ref().unwrap(),
                config.chat_id.as_ref().unwrap(),
                article,
            )
            .await
            {
                Ok(_) => println!("Message sent successfully"),
                Err(e) => println!("Error sending message: {}", e),
            }
        }
        Mode::Http => {
            match client
                .post("http://localhost:3030/endpoint")
                .json(&article)
                .send()
                .await
            {
                Ok(_) => println!("Article posted successfully"),
                Err(e) => println!("Error posting article: {}", e),
            };
        }
    }
}
