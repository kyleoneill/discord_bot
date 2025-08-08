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
    pokemon_type TEXT NOT NULL,
    classification TEXT NOT NULL,
    PRIMARY KEY (username, pokemon_name, pokemon_type),
    FOREIGN KEY(username) REFERENCES users(discord_username) ON DELETE CASCADE
);
