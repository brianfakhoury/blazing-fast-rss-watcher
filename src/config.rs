use dotenv::dotenv;
use std::env;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Blazing Fast RSS Watcher",
    about = "A blazing fast RSS watcher."
)]
struct Opt {
    #[structopt(subcommand)]
    mode: Mode,
    /// Specify the IP address to listen on for websub
    #[structopt(short = "a", long = "address", default_value = "")]
    ip_addr: String,
    /// Specify the poll interval in milliseconds
    #[structopt(short = "p", long = "poll", default_value = "20000", global = true)]
    poll_interval: u64,
    /// xit program immediately after one article per source, good for testing
    #[structopt(short = "o", long = "one", global = true)]
    one_at_a_time: bool,
}

#[derive(Debug, StructOpt)]
pub enum Mode {
    #[structopt(about = "Run in test mode.")]
    Test,
    #[structopt(about = "Run in telegram mode.")]
    Telegram,
    #[structopt(about = "Run in http mode.")]
    Http,
}

pub struct AppConfig {
    pub mode: Mode,
    pub chat_id: Option<String>,
    pub bot_token: Option<String>,
    pub ip_address: Option<String>,
    pub poll_interval: u64,
    pub one_at_a_time: bool,
}

impl AppConfig {
    pub fn from_env_args() -> Result<AppConfig, Box<dyn Error>> {
        let opt = Opt::from_args();

        let mode = opt.mode;
        let ip_address = Some(opt.ip_addr).filter(|s| !s.is_empty());
        let poll_interval = opt.poll_interval;
        let one_at_a_time = opt.one_at_a_time;
        let mut chat_id = None;
        let mut bot_token = None;

        match mode {
            Mode::Test | Mode::Http => {}
            Mode::Telegram => {
                dotenv().ok();
                chat_id = Some(
                    env::var("CHAT_ID").expect("CHAT_ID not found. Please set it in the .env file"),
                );
                bot_token = Some(
                    env::var("BOT_TOKEN")
                        .expect("BOT_TOKEN not found. Please set it in the .env file"),
                );
            }
        }

        Ok(AppConfig {
            mode,
            chat_id,
            bot_token,
            ip_address,
            poll_interval,
            one_at_a_time,
        })
    }
}
