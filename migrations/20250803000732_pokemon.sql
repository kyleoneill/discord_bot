CREATE TABLE IF NOT EXISTS pokemon_command_record (
    username TEXT PRIMARY KEY NOT NULL,
    last_used_at INTEGER NOT NULL DEFAULT 0,
    times_used INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(username) REFERENCES users(discord_username) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pokemon_ownership_record (
    username TEXT NOT NULL,
    pokemon_name TEXT NOT NULL,
    number_owned INTEGER NOT NULL DEFAULT 0,
    shiny BOOLEAN NOT NULL DEFAULT 0,
    april_fools BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (username, pokemon_name, shiny, april_fools),
    FOREIGN KEY(username) REFERENCES users(discord_username) ON DELETE CASCADE
);
