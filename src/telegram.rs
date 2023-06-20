use crate::model::ArticleInfo;
use regex::Regex;
use reqwest::Client;
use std::error::Error;

pub async fn send_message(
    bot_token: &str,
    chat_id: &str,
    article: &ArticleInfo,
) -> Result<(), Box<dyn Error>> {
    let message = format!(
        "<b>{}</b>{}",
        article.title,
        article
            .description
            .as_ref()
            .map(|s| format!("\n\n{}", s))
            .unwrap_or(String::from(""))
    );
    let parse_mode = String::from("HTML");
    let disable_web_page_preview = String::from("true");
    let domain = Regex::new(r#"^https?://(?:www\.)?([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})(?:/|$)"#)?
        .captures(&article.link)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or_default();
    let reply_markup = format!(
        "{{\"inline_keyboard\":[[{{\"text\":\"Open Article on {}\",\"url\":\"{}\"}}]]}}",
        domain, article.link
    );
    let client = Client::new();
    client
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            bot_token
        ))
        .form(&[
            ("chat_id", chat_id),
            ("text", &message),
            ("parse_mode", &parse_mode),
            ("disable_web_page_preview", &disable_web_page_preview),
            ("reply_markup", &reply_markup),
        ])
        .send()
        .await?;

    Ok(())
}
