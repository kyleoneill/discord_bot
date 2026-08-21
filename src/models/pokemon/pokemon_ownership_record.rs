use serde::{Deserialize, Serialize};

use super::PokemonType;

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonOwnershipRecord {
    pub discord_id: String,
    pub pokemon_slug: String,
    pub number_owned: i64,
    pub pokemon_type: PokemonType,
}

pub struct PokemonOwnershipRecordPK {
    pub discord_id: String,
    pub pokemon_slug: String,
    pub pokemon_type: PokemonType,
}
