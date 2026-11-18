# KYOKA 4.0 — Greed Feature Parity Roadmap

This is an independent implementation inspired by the public feature categories and workflows of Greed. Do not copy Greed's source code, private APIs, branding, artwork, or proprietary implementation. KYOKA keeps its own Aizen identity, commands, data model and user experience.

## Product areas to deliver

| Area | Required behaviour | Existing KYOKA foundation | 4.0 work |
|---|---|---|---|
| Security | Antinuke limits, whitelist/admins, anti-raid detection, lockdown/recovery, honeypot, verification | antinuke and automod cogs/managers | Audit safety thresholds, make actions atomic, add raid signals and recovery tests |
| Moderation | Ban/kick/timeout/warn/cases, hierarchy checks, appeals, modstats, logs | moderation cogs and logging | Unified case service, approvals, appeals and full audit history |
| Tickets | Multi-panel buttons/selects, forms, claims, transfers, SLA/status board, transcripts, automations | ticket system and ticket manager | Preserve existing engine; add workflow editor and dashboard parity |
| Automation | Rules/workflows, scheduled posts, autoresponders, reminders, bump reminders | utility cogs and task manager | Durable queue, trigger/action registry and safe execution limits |
| Server management | Role/channel tools, autoroles, reaction/button roles, custom embeds, welcome/leave, counters | utility, admin and branding modules | Consistent setup wizard, permission preflight and dashboard controls |
| Community | Levels, XP decay/rewards, streaks, starboard, polls, giveaways, suggestions, birthdays | leveling, giveaway and utility modules | Unified settings, anti-abuse and reliable scheduled jobs |
| Voice | VoiceMaster/join-to-create, music, 24/7 channel, queue, autoplay and controls | music and temp voice modules | Multi-node Lavalink routing, voice session recovery and per-guild isolation |
| Economy and fun | Wallet, daily/work/transfer, leaderboards, games, roleplay and image tools | economy, games and fun cogs | Shared transaction ledger, anti-exploit limits and modular command packs |
| Integrations | Last.fm, Spotify metadata, social feeds and platform notifications | tech feeds and music integrations | Provider adapters, rate limits, retries and user-controlled data |
| Dashboard | OAuth login, per-server setup, live tickets, logs, analytics, permissions and premium | Cloudflare Pages/API functions | Complete feature coverage, secure API, live updates and mobile-first UI |
| Platform | Prefix and slash commands, localization, premium, support and analytics | command registry, premium manager | Unified command registry, feature flags and migration-safe entitlements |

## Delivery sequence

### Phase 0 — Safety baseline
- Keep the source repository and its production deployment untouched.
- Add regression tests around current commands, ticket lifecycle, antinuke permissions and database migrations.
- Inventory every existing command and dashboard endpoint.
- Remove secrets from tracked configuration; rotate any credential that was ever committed.

### Phase 1 — Runtime and data foundation
- Configurable Discord auto-sharding (started).
- Worker identity and disjoint shard assignment validation.
- Shared Redis cache/pubsub and distributed rate-limit primitives.
- Durable queue with idempotent job execution.
- Database migrations, indexes, pool budgets and backup/restore drills.
- Structured logs, metrics, tracing and health/readiness endpoints.

### Phase 2 — Greed-style server control
- One consistent setup experience for security, moderation, logging, tickets and onboarding.
- Complete permission preflight and safe defaults.
- Dashboard settings for every configurable feature.
- Audit history and clear failure messages.

### Phase 3 — Community and engagement
- Leveling, economy, giveaways, role tools, feeds, VoiceMaster and fun modules.
- Scheduled work moved off gateway event handlers.
- Per-guild quotas and anti-abuse protections.

### Phase 4 — Scale and launch
- Staging bot, migration rehearsal and canary rollout.
- Load/soak/failover tests.
- Measure and publish capacity before any public scale claim.
- Roll out gradually with rollback controls.

## Definition of done

A module is not complete merely because a command exists. It must have:
- Permission and hierarchy checks.
- Per-guild settings with safe defaults.
- Persistent state and migration coverage where needed.
- Dashboard controls where applicable.
- Rate limits, audit logs and user-friendly errors.
- Unit and integration tests, including failure/restart cases.
- Documentation and a verified deployment path.
