CREATE TABLE IF NOT EXISTS guild_settings (
    guild_id BIGINT PRIMARY KEY,
    prefix TEXT NOT NULL DEFAULT '+',
    welcome_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    welcome_channel_id BIGINT,
    welcome_message TEXT,
    log_channel_id BIGINT,
    tickets_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ticket_category_id BIGINT,
    anti_nuke_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    anti_nuke_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    ai_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ai_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    economy_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS kyoka_user_balances (
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0 CHECK (balance >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id, user_id)
);

CREATE TABLE IF NOT EXISTS kyoka_economy_transactions (
    id UUID PRIMARYKEY DEFAULT gen_random_uuid(),
    guild_id BIGINT NOT NULL,
    actor_id BIGINT NOT NULL,
    target_id BIGINT,
    amount BIGINT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
