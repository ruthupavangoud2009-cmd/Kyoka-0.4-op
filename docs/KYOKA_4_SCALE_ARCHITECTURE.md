# KYOKA 4.0 — Scale and Reliability Architecture

## Capacity goal

The product goal is to support communities containing at least one million members in aggregate and to engineer toward very high simultaneous activity. This is a capacity target, not a claim that the current deployment can serve one million live users today.

A Discord bot cannot independently guarantee a fixed number of concurrent users. Real capacity depends on Discord gateway shard assignments and intents, event rate, command mix, hosting CPU/network limits, database capacity, third-party APIs, and Discord rate limits. Capacity must be measured with repeatable load tests and production telemetry.

## Target topology

```text
Discord Gateway
   |
Gateway shard workers (many processes, disjoint shard IDs)
   |
Command/event handlers (bounded concurrency, fast acknowledgement)
   |                 |                    |
Redis cache/pubsub   Durable job queue    Metrics + tracing
   |                 |                    |
PostgreSQL primary + read replicas / partitioned high-volume tables
   |
Object storage for transcripts, exports and generated media

Dashboard/API (stateless replicas) -> OAuth/session store -> same services
```

### Required engineering rules

1. **Shard ownership:** every shard ID belongs to exactly one live worker. Workers use the same configured total shard count and disjoint SHARD_IDS. Never run duplicate ownership of the same shard.
2. **No process-local truth:** caches are disposable. Guild settings, premium entitlements, ticket state, moderation cases and scheduled work are persisted. Cross-worker invalidation/events use Redis or an equivalent shared service.
3. **Bounded work:** command handlers acknowledge interactions quickly, apply per-guild/user limits, and move slow work to bounded workers. No unbounded task creation or unbounded in-memory queues.
4. **Database budgets:** calculate worker_count × DB_POOL_MAX_SIZE against the database connection budget. Use migrations, indexes, query timeouts, retry budgets and idempotent writes.
5. **Durable jobs:** reminders, feeds, giveaways, transcript exports and scheduled workflows use a durable queue with retries, deduplication, dead-letter handling and idempotency keys.
6. **Failure isolation:** circuit-break external providers, cap concurrency, use exponential backoff with jitter, and avoid retry storms.
7. **Observability:** expose shard health, gateway reconnects, event-loop lag, command latency percentiles, queue age/depth, DB pool wait time, cache hit rate, error rate and per-provider failures.
8. **Security:** least-privilege permissions, secret manager/environment secrets, OAuth state validation, audit logs, per-server data isolation and tested backup/restore.

## Current foundation in this repository

- discord.py bot now uses commands.AutoShardedBot.
- SHARD_COUNT can pin the total gateway shard count; blank means library/Discord auto-discovery.
- SHARD_IDS can assign a disjoint shard subset to a worker process.
- DB_POOL_MIN_SIZE and DB_POOL_MAX_SIZE configure each process's PostgreSQL pool.
- These changes enable controlled gateway sharding and pool budgeting; they do not yet provide distributed coordination, shared cache, a durable queue, autoscaling, or one-million-user capacity.

## Deployment configuration

Single worker / automatic shard discovery:

```env
SHARD_COUNT=
SHARD_IDS=
DB_POOL_MIN_SIZE=1
DB_POOL_MAX_SIZE=10
```

Multi-worker deployment example (illustrative only; use the actual shard count recommended for the application):

Worker A:
```env
SHARD_COUNT=8
SHARD_IDS=0,1
```

Worker B:
```env
SHARD_COUNT=8
SHARD_IDS=2,3
```

Continue assigning every shard exactly once. The example is not a production capacity recommendation. Do not scale workers by copying the same SHARD_IDS to each instance.

## Release gates before claiming scale

- Unit tests for shard parsing, worker ownership and idempotency.
- Integration tests against a staging Discord application and isolated database.
- Load tests for realistic message, interaction, join/leave, ticket and logging mixes.
- Soak tests and controlled worker restarts while jobs are queued.
- Verify no duplicate scheduled jobs or duplicate shard ownership.
- Publish measured capacity, p50/p95/p99 latency, error rate and resource use. Never infer capacity from guild count alone.
