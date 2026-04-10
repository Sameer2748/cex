-- Create Users Table (Postgres Style)
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create Balances Table 
-- Postgres doesn't have "UNSIGNED", so we use BIGINT
CREATE TABLE IF NOT EXISTS balances (
    user_id INTEGER NOT NULL REFERENCES users(id),
    asset TEXT NOT NULL,
    amount BIGINT NOT NULL DEFAULT 0, 
    PRIMARY KEY (user_id, asset),
    CONSTRAINT positive_balance CHECK (amount >= 0) -- Security check
);

-- Create Trade History Table
CREATE TABLE IF NOT EXISTS trades (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL,
    price BIGINT NOT NULL,
    qty BIGINT NOT NULL,
    maker_id INTEGER REFERENCES users(id),
    taker_id INTEGER REFERENCES users(id),
    timestamp BIGINT NOT NULL
);
