-- 1) markets table - stores all 4-hour prediction markets
CREATE TABLE IF NOT EXISTS markets (
    id BIGSERIAL PRIMARY KEY,
    market_id BIGINT NOT NULL UNIQUE,
    asset TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    lock_time TIMESTAMPTZ NOT NULL,

    open_price NUMERIC(30,10),
    close_price NUMERIC(30,10),

    green_pool_weighted NUMERIC(30,10) DEFAULT 0,
    red_pool_weighted NUMERIC(30,10) DEFAULT 0,
    virtual_liquidity NUMERIC(30,10) DEFAULT 100,

    settled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2) bets table - stores user bets for each market
CREATE TABLE IF NOT EXISTS bets (
    id BIGSERIAL PRIMARY KEY,
    wallet TEXT NOT NULL,
    market_id BIGINT NOT NULL REFERENCES markets(market_id) ON DELETE CASCADE,

    side TEXT NOT NULL CHECK (side IN ('GREEN','RED')),
    amount NUMERIC(30,10) NOT NULL,
    weight NUMERIC(5,2) NOT NULL,
    effective_stake NUMERIC(30,10) NOT NULL,

    payout NUMERIC(30,10) DEFAULT 0,
    claimed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3) pnl table - optional analytics table
CREATE TABLE IF NOT EXISTS pnl (
    wallet TEXT PRIMARY KEY,
    total_pnl NUMERIC(30,10) DEFAULT 0,
    total_bets BIGINT DEFAULT 0,
    win_rate NUMERIC(5,2) DEFAULT 0,
    last_updated TIMESTAMPTZ DEFAULT NOW()
);

-- Useful indexes
CREATE INDEX IF NOT EXISTS idx_markets_time ON markets (asset, start_time);
CREATE INDEX IF NOT EXISTS idx_bets_market ON bets (market_id);
