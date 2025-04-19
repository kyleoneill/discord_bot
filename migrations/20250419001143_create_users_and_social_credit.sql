-- Create the users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    discord_username TEXT NOT NULL UNIQUE
);

-- Create the social_credit table
CREATE TABLE IF NOT EXISTS social_credit (
    username TEXT PRIMARY KEY NOT NULL,
    positive_credit INTEGER NOT NULL DEFAULT 0,
    negative_credit INTEGER NOT NULL DEFAULT 0,
    traded_credit INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(username) REFERENCES users(discord_username) ON DELETE CASCADE
);
