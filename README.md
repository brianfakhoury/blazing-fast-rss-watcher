# Blazing Fast RSS Watcher

⚠️ This codebase is not yet ready for production usage.

This can be used as a library, or as a standalone binary.

## How to run it as a binary

1. `$ git clone` this repository on the main branch
2. `$ cargo build --release`
3. Place your rss links in the `rss_feeds.txt` file, one per line
4. `$ target/release/blazing-fast-rss-watcher test`

Available modes: `test`, `http`, `telegram`

Optionally:

- Create a `.env` file with `BOT_TOKEN` and `CHAT_ID` for posting to Telegram (required in `telegram` mode)

## How to use the library

1. `cargo add blazing-fast-rss-watcher`
2. See `main.rs` for an example. The `process_rss_feeds` function is the main entrypoint. The `config::AppConfig` struct is the main configuration struct, which is the parameter to `process_rss_feeds`.
3. You can configure the environment using the `::from_env()` method on the `AppConfig` struct.

## TODO

- Implement WebSub detection for even faster/more efficient operation for supported feeds
- Docker image
- Add param for http mode variable address
- Make Article struct more dyanmic, allow custom patterns for the description.
