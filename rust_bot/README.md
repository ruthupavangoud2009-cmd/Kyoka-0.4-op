# KYOKA — Rust rewrite

The target runtime is **Rust-only**. The Python backup is a feature reference and is not intended to be part of the Rust deployment/runtime.

## Rust foundation
- Discord Gateway via Serenity
- Tokio async runtime
- Axum health endpoints at `/` and `/health`
- Config parsing and environment helpers
- Ping argument parsing
- Member-count formatting
- Bot-info and telemetry formatting
- Guild-independent music queue state (enqueue, advance, repeat modes, shuffle, previous, multi-skip, remove, move, clear, stop)
- PostgreSQL connection pool and migration runner, with initial guild settings and economy tables
- Anti-nuke threshold/window tracker (event types and per-actor counts; Discord audit-log enforcement still pending)
- Ticket lifecycle registry (create, claim, close; Discord channel creation and persistence integration still pending)

## Build / run
From `rust_bot/`:
```sh
cargo build
BOT_TOKEN=... cargo run
```

Set `PORT` to override the health-server port (default 8080), and `BOT_PREFIX` or `DEFAULT_PREFIX` for the command prefix.

## Migration status
This branch is a work in progress and **not yet a complete or verified replacement bot**. Queue state is implemented as transport-independent Rust logic; Lavalink connection/player events and Discord voice integration are still pending. Other systems still need full ports and integration: moderation enforcement/Anti-Nuke audit-log actions, tickets (Discord channels, permissions, transcripts and persistence), AI/voice, TOP AI/economy logic, database-backed repositories, dashboard/OAuth/API, and remaining commands/events/tasks. The new database, Anti-Nuke tracker, and ticket lifecycle modules are foundations—not production-connected features yet.

The Python backup is not being maintained as a parallel runtime. Use it only as a reference for behavior while implementing the complete Rust version. Do not deploy this as a full replacement until the Rust build succeeds and the feature migration is verified.
