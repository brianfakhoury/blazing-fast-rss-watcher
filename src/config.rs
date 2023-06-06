use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct AppConfig {
    pub mode: String,
    pub chat_id: Option<String>,
    pub bot_token: Option<String>,
}

impl AppConfig {
    pub fn from_env_args() -> Result<AppConfig, Box<dyn Error>> {
        let mode = env::args().nth(1).unwrap_or_default();

        let mut chat_id = None;
        let mut bot_token = None;

        match mode.as_str() {
            "test" | "http" => {}
            "telegram" => {
                dotenv().ok();
                chat_id = Some(
                    env::var("CHAT_ID").expect("CHAT_ID not found. Please set it in the .env file"),
                );
                bot_token = Some(
                    env::var("BOT_TOKEN")
                        .expect("BOT_TOKEN not found. Please set it in the .env file"),
                );
            }
            _ => {
                eprintln!(
                    "Invalid mode: {}. Please choose one of: test, telegram, http",
                    mode
                );
                std::process::exit(1);
            }
        }

        Ok(AppConfig {
            mode,
            chat_id,
            bot_token,
        })
    }
}
