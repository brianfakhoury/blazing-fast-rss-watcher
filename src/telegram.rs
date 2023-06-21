use crate::model::Article;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

fn clean_html_to_plain_text(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("p").unwrap();
    match fragment.select(&selector).next() {
        Some(p) => p.text().collect::<Vec<_>>().join(""),
        None => html.into(),
    }
}

pub async fn send_message(
    client: &Client,
    bot_token: &str,
    chat_id: &str,
    article: &Article,
) -> Result<(), Box<dyn Error>> {
    let desc = clean_html_to_plain_text(article.description.as_ref().unwrap_or(&String::from("")));
    let message = format!("<b>{}</b>\n\n{}", article.title, desc);
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
