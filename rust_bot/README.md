# KYOKA Rust port (foundation)

This is a **parallel Rust implementation area**. Existing Python files remain untouched.

## Current foundation
- Discord gateway connection using Serenity
- Environment token support: `BOT_TOKEN` or `DISCORD_TOKEN`
- Prefix from `DEFAULT_PREFIX` (defaults to `?`)
- Basic `ping` and `rustinfo` test commands
- HTTP health endpoints `/` and `/health` on `PORT` (defaults to 8080)

## Build/run
From this directory:
```sh
cargo build
BOT_TOKEN=... cargo run
```

## Migration status
This is not yet a complete port. The Python bot has hundreds of modules, including moderation, music, tickets, economy, AI/voice, giveaways, utilities, and dashboard APIs. Port each feature in parity, migrate persistence and configuration, and add Rust tests before production cutover. Do not remove or replace the Python implementation until parity and validation are complete.
