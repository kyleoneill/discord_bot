CREATE TABLE IF NOT EXISTS pokemon_command_record (
    discord_id TEXT PRIMARY KEY NOT NULL,
    last_used_at INTEGER NOT NULL DEFAULT 0,
    times_used INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(discord_id) REFERENCES users(discord_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pokemon_ownership_record (
    discord_id TEXT NOT NULL,
    pokemon_slug TEXT NOT NULL,
    number_owned INTEGER NOT NULL DEFAULT 0,
    pokemon_type TEXT NOT NULL, -- Options are 'Normal', 'Shiny', 'AprilFools'
    PRIMARY KEY (discord_id, pokemon_slug, pokemon_type),
    FOREIGN KEY (discord_id) REFERENCES users(discord_id) ON DELETE CASCADE
);
