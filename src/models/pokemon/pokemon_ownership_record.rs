use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serenity::model::Color;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
pub enum PokemonType {
    Normal,
    Shiny,
    AprilFools,
}

impl From<String> for PokemonType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Normal" => Self::Normal,
            "Shiny" => Self::Shiny,
            "AprilFools" => Self::AprilFools,
            _ => panic!("Failed to convert a string to a pokemon type"),
        }
    }
}

impl PokemonType {
    pub fn get_link_for_type(&self, pokemon_name: &str) -> String {
        match self {
            Self::Normal => format!(
                "https://play.pokemonshowdown.com/sprites/ani/{}",
                pokemon_name
            ),
            Self::AprilFools => format!(
                "https://play.pokemonshowdown.com/sprites/afd/{}",
                pokemon_name
            ),
            Self::Shiny => format!(
                "https://play.pokemonshowdown.com/sprites/ani-shiny/{}",
                pokemon_name
            ),
        }
    }
    pub fn get_display_text_for_type(&self, pokemon_name: String) -> String {
        match self {
            Self::Normal => format!("you caught a **{}**", pokemon_name),
            Self::AprilFools => format!("April Fools! You caught a **{}**", pokemon_name),
            Self::Shiny => format!("you caught a shiny **{}**", pokemon_name),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
pub enum PokemonClassification {
    Normal,
    Legendary,
    Mythical,
}

impl PokemonClassification {
    pub fn get_color_for_embed(&self) -> Color {
        match self {
            PokemonClassification::Normal => Color::LIGHT_GREY,
            PokemonClassification::Legendary => Color::GOLD,
            PokemonClassification::Mythical => Color::BLITZ_BLUE,
        }
    }
}

// This is used by sqlx
impl From<String> for PokemonClassification {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Normal" => Self::Normal,
            "Legendary" => Self::Legendary,
            "Mythical" => Self::Mythical,
            _ => panic!("Failed to convert a string to a pokemon classification"),
        }
    }
}

// This is used to read from the input file
impl FromStr for PokemonClassification {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "legendary" => Ok(Self::Legendary),
            "mythic" => Ok(Self::Mythical),
            _ => Err(format!(
                "Got an invalid value while parsing a PokemonClassification: {}",
                s
            )),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonOwnershipRecord {
    pub username: String,
    pub pokemon_name: String,
    pub number_owned: i64,
    pub pokemon_type: PokemonType,
    pub classification: PokemonClassification,
}

pub struct PokemonOwnershipRecordPK {
    pub username: String,
    pub pokemon_name: String,
    pub pokemon_type: PokemonType,
}

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub classification: PokemonClassification,
}
