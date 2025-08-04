use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonOwnershipRecord {
    pub username: String,
    pub pokemon_name: String,
    pub number_owned: i64,
    pub shiny: bool,
    pub april_fools: bool,
}

pub struct PokemonOwnershipRecordPK {
    pub username: String,
    pub pokemon_name: String,
    pub shiny: bool,
    pub april_fools: bool,
}
