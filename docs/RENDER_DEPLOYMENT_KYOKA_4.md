# Deploy KYOKA 4.0 on Render

This repository is configured as a Render Python web service. The web process starts the health/API server and the Discord bot in the same process.

## Deploy from an Android phone

1. Open the repository: https://github.com/ruthupavangoud2009-cmd/Kyoka-0.4-op
2. Open Render and sign in: https://dashboard.render.com/
3. Choose **New** -> **Blueprint** and connect the GitHub repository above.
4. Select the `main` branch and deploy the Blueprint. Render reads `render.yaml`, installs `requirements.txt` through `build.sh`, then runs `python -m src.main`.
5. In the new service's **Environment** tab, add the required secrets and settings below, then save/redeploy.

## Required environment variables

- `DISCORD_TOKEN`: token for the dedicated KYOKA 4.0 Discord application. Never paste it into GitHub or chat.
- `DATABASE_URL`: PostgreSQL connection string. Use a reachable PostgreSQL provider and its recommended connection-pooler URL if required by your host.
- `DISCORD_CLIENT_ID`: application ID for the same KYOKA 4.0 Discord application.
- `DISCORD_CLIENT_SECRET`: OAuth2 client secret for that application.
- `DISCORD_REDIRECT_URI`: after Render creates the service, set this to `https://YOUR-RENDER-SERVICE.onrender.com/api/auth/callback`.
- `FRONTEND_ORIGIN`: set to the exact public origin serving the dashboard. If the dashboard is served by this same Render service, use `https://YOUR-RENDER-SERVICE.onrender.com`.
- `DASHBOARD_SECRET_KEY`: keep the generated value from Render; do not publish it.

Set the exact same OAuth redirect URI in Discord Developer Portal -> KYOKA 4.0 application -> OAuth2 -> Redirects. Do not reuse the Aizen production application's client ID, secret, token, or redirect URL.

## Useful optional settings

- `BOT_PREFIX=+`
- `SYNC_COMMANDS_ON_STARTUP=false` initially; sync commands deliberately after verifying the bot is online.
- `SHARD_COUNT` and `SHARD_IDS`: leave unset for a single Render service unless you have planned a multi-worker shard deployment. Multiple workers must own unique shard IDs with the same total shard count.
- `DB_POOL_MIN_SIZE=1`, `DB_POOL_MAX_SIZE=5`: per-process pool sizes. Keep the sum across all processes below the database provider's connection limit.
- `GEMINI_API_KEY`: optional AI integration key.

## Verify the deployment

- Open `https://YOUR-RENDER-SERVICE.onrender.com/health`. A healthy response confirms the HTTP health server is responding; it does not by itself prove the Discord gateway is connected.
- Open Render -> service -> **Logs**. Confirm successful Discord login, shard readiness, and database connection.
- Invite the bot using the invite URL generated for the KYOKA 4.0 application. Grant only the permissions its enabled modules require; avoid Administrator unless you deliberately choose it.
- Test one low-risk command, then test moderation/tickets in a private staging server before inviting it elsewhere.

## Render plan and scale notes

The Blueprint currently uses Render's Free plan setting. Free web services can spin down after inactivity and have limited CPU/RAM; they are suitable for initial testing, not a guaranteed always-on production Discord bot. Use an always-on paid instance or another suitable host for reliable 24/7 operation.

The 100,000-guild goal is not achieved by this one-service Blueprint. It requires a tested multi-shard worker deployment, Redis/shared coordination, durable job queues, database capacity planning, monitoring, and measured load/soak tests. Do not set arbitrary shard counts or run duplicate workers with overlapping shard IDs.

## Start command

The configured Render start command is:

```bash
python -m src.main
```

For a local shell with dependencies and environment variables configured, the same command starts KYOKA.
