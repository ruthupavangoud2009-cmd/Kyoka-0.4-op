# KYOKA 4.0 — Product, Command-Parity & Scale Acceptance Specification

This document defines the target for an independently implemented Discord community bot inspired by publicly documented feature categories and workflows from mature multipurpose bots. It does not copy another product's source code, private APIs, branding, or artwork.

## Product target

KYOKA 4.0 should feel like a complete, independently branded Discord bot with a consistent command system, reliable configuration, and a dashboard/control plane. Existing KYOKA commands are not considered complete merely because a command name exists: every command must be tested end-to-end, persist its settings, enforce permissions, handle Discord API failures, and provide useful feedback.

## Discord bot identity

- Operate as a normal Discord bot application with its own application ID, bot user, token, avatar, username, banner where Discord permits, bio/about text where Discord permits, and global activity/status.
- Show the KYOKA identity consistently in help, embeds, dashboard, invite page, logs, and support links.
- Provide owner-only identity management for supported global bot profile fields. Explain Discord restrictions and do not pretend a global profile can be independently changed for each server.
- Use a dedicated application and token supplied by the owner. Never commit tokens or secrets.
- Keep server-specific branding (welcome art, embed theme, nicknames where permitted) separate from the global bot account identity.

## Command and feature parity scope

All modules below require both a clear user-facing setup path and complete runtime behavior. Prefix and slash interfaces should be consistent where Discord's command limits and interaction model allow. Commands must be discoverable in paginated help with examples, permissions, aliases, and setup prerequisites.

### Security and protection
- Antinuke: configurable per-action thresholds, trusted users/roles, extra owners, punishment policy, incident log channel, cooldowns, audit-log correlation, safe mode, and lockdown/recovery.
- Antiraid: join velocity detection, account-age/join-age rules, verification gates, quarantine, raid mode, and false-positive controls.
- Automod: spam, mention spam, invite/link rules, keyword/regex filters, caps, emoji/repetition, attachments, scam patterns, exemptions, escalating actions, and audit trail.
- Verification: button or modal flows, role assignment, timeout/quarantine, logging, and configurable bypass rules.
- Security actions must be idempotent, permission checked, rate-limited, and fail safely when audit logs or permissions are unavailable.

### Moderation and server administration
- Ban/unban, kick, timeout/untimeout, warn/warnings, purge, softban where appropriate, lock/unlock, slowmode, role and channel management, nickname controls, mod notes, case history, bulk actions, and moderation logs.
- Per-command role restrictions, ignored channels, command toggles, staff hierarchy checks, reason capture, confirmation for destructive bulk actions, and consistent case IDs.
- Message edit/delete logs, member join/leave/update, role/channel changes, voice-state logs, and configurable privacy/retention.

### Tickets and support
- Multiple ticket panels, buttons/select menus, forms/modals, categories, private channels or threads, staff routing, claim/unclaim, close/reopen, transcript export, feedback, limits, cooldowns, access requirements, support hours, auto-close, and ticket analytics.
- Optional AI summaries and suggested follow-ups must be disabled by default, explicitly enabled by an authorized server administrator, and never block ticket creation or closing.
- Persist ticket state and recover open tickets after restart.

### Automation and engagement
- Welcome/leave messages and cards, autoroles, booster roles, reaction/button roles, sticky messages, suggestions, polls, giveaways, starboard, invite tracking, reminders, scheduled posts, counters, birthdays, bump reminders, and custom embeds.
- Every automation needs per-server settings, validation, safe retries, duplicate prevention, and a clear disable/reset command.

### Community and utility
- AFK, user/server/channel/role information, avatar/banner, member count, server stats, custom commands/responses, autoresponders, translation, dictionary, weather integration, screenshot utilities where safe, message snipe where configured, and privacy controls.
- Leveling and XP remain opt-in per server and disabled by default until explicitly enabled.

### Voice, music, and media
- Music playback, queue, playlists, filters, autoplay, controls, DJ permissions, 24/7 voice opt-in, and robust Lavalink node health/reconnect handling.
- Voice channels: temporary voice rooms, owner controls, limits, privacy, region/settings where supported, and cleanup.
- Welcome/TTS/AI voice features must use bounded queues and independent voice-session state so one guild's voice workload cannot block another.
- External media providers must be isolated behind adapters with retries, timeouts, and clear unsupported-source messages.

### Economy, fun, and integrations
- Optional server-scoped economy with balances, daily/weekly rewards, shop, inventory, leaderboards, and anti-abuse transaction rules.
- Games, social/fun commands, anime interactions, image tools, and integrations such as music scrobbling or social feeds where credentials and provider terms permit.
- Economy and leveling are separate opt-in modules; no hidden XP/economy collection.

### Dashboard and management
- OAuth2 login with strict state/PKCE validation, secure sessions, CSRF protection, and server selection based on actual permissions.
- Live per-server configuration for all supported modules, command restrictions, ticket panels, logs, roles, automations, and premium entitlements.
- Bot health, shard status, guild/member counts, queue depth, error rates, latency percentiles, database pool use, Redis health, and recent incidents.
- Dashboard changes must validate, persist transactionally, invalidate shared caches, and be reflected by the bot without restart where safe.
- Owner control plane for deployment health, shard/worker status, global bans, feature flags, migrations, and incident response.

## Scale target: 100,000 guilds

100,000 guilds is a capacity objective, not a claim that the current code or a single free hosting instance can serve that load. "No lag" is not a measurable guarantee. The release target must be defined by measured latency/error/availability SLOs under representative load.

Required architecture:
1. Multiple gateway shard workers with explicit, non-overlapping shard ownership and safe rolling restarts.
2. Redis-backed shared cache/pub-sub, distributed rate limits, leases/locks, and worker coordination; local memory remains only an L1 cache.
3. PostgreSQL with indexed, tenant-scoped tables, migrations, bounded per-worker pools, query timeouts, and a documented total connection budget.
4. Durable background queues for scheduled jobs and expensive operations, with retries, backoff, dead-letter handling, idempotency keys, and per-guild fairness.
5. Stateless dashboard/API processes behind a load balancer; no session or configuration state held only in one process.
6. Bounded concurrency and timeouts for Discord API, external providers, AI, media extraction, and database calls.
7. Metrics, structured logs, tracing/correlation IDs, alerts, backups, restore drills, and graceful degradation.
8. Load, soak, failover, shard-rebalance, cache-outage, database-outage, and queue-recovery tests before claiming capacity.

## Definition of done

A feature is complete only when:
- Its prefix/slash commands and dashboard paths (where applicable) are implemented and documented.
- Authorization, permission hierarchy, input validation, cooldowns, and error handling are tested.
- Settings survive restarts and are isolated by guild.
- It has unit tests plus integration tests for success, permission denial, missing permissions, API failure, retries, and restart recovery.
- It emits useful logs/metrics without leaking secrets or message contents beyond configured retention.
- It is included in paginated help and a generated command inventory.
- It is verified on a staging bot before production rollout.

A scale milestone is complete only when a repeatable load-test report documents guild/shard topology, event mix, concurrency, host resources, p50/p95/p99 latency, error rate, queue lag, database utilization, and recovery behavior. Do not advertise 100,000-guild capacity until this evidence exists.

## Delivery sequence

1. Audit every existing cog and command against this specification; mark implemented, partial, broken, missing, or unsafe.
2. Fix startup, extension loading, permissions, persistence, and existing command regressions.
3. Implement shared Redis infrastructure, worker coordination, durable queues, and distributed rate limiting.
4. Complete security, moderation, tickets, and automation workflows.
5. Complete utility, community, voice/music, economy, and integrations.
6. Complete OAuth dashboard and owner control plane.
7. Run staged load/soak/failure tests; tune and publish verified capacity results.
