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

## TODO

- Implement WebSub detection for even faster/more efficient operation for supported feeds
- Docker image
