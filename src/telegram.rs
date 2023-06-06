use reqwest::Client;
use std::error::Error;

pub async fn send_message(
    client: &Client,
    bot_token: &str,
    chat_id: &str,
    title: &str,
    description: &str,
    link: &str,
) -> Result<(), Box<dyn Error>> {
    let message = format!("<b>{}</b>\n\n{}", title, description);
    let parse_mode = String::from("HTML");
    let disable_web_page_preview = String::from("true");
    let reply_markup = format!(
        "{{\"inline_keyboard\":[[{{\"text\":\"Open Article on Coindesk\",\"url\":\"{}\"}}]]}}",
        link
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
