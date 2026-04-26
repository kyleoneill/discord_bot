-- Create the users table
CREATE TABLE IF NOT EXISTS users (
    discord_id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE
);

-- Create the social_credit table
CREATE TABLE IF NOT EXISTS social_credit (
    discord_id TEXT PRIMARY KEY NOT NULL,
    positive_credit INTEGER NOT NULL DEFAULT 0,
    negative_credit INTEGER NOT NULL DEFAULT 0,
    traded_credit INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(discord_id) REFERENCES users(discord_id) ON DELETE CASCADE
);
