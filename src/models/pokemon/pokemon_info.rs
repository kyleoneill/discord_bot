use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serenity::model::Color;

// POKEMON TYPE
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
    pub fn get_link_for_type(&self, pokemon: &Pokemon) -> String {
        let pokemon_link_name: String = {
            let lowercase_name = pokemon.name.to_lowercase();
            lowercase_name.replace(' ', "")
        };
        match self {
            Self::Normal => format!("https://play.pokemonshowdown.com/sprites/ani/{}.gif", pokemon_link_name),
            Self::AprilFools => format!("https://play.pokemonshowdown.com/sprites/afd/{}.png", pokemon_link_name),
            Self::Shiny => format!("https://play.pokemonshowdown.com/sprites/ani-shiny/{}.gif", pokemon_link_name),
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

// POKEMON STATS
#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize)]
pub struct PokemonStats {
    hp: u32,
    atk: u32,
    def: u32,
    spa: u32,
    spd: u32,
    spe: i32,
}

impl Display for PokemonStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} HP\n{} Attack\n{} Defense\n{} Special Attack\n{} Special Defense\n{} Speed",
            self.hp, self.atk, self.def, self.spa, self.spd, self.spe
        )
    }
}

// POKEMON RARITY
#[derive(Debug, Default, Deserialize, Serialize, Clone, sqlx::Type)]
pub enum PokemonRarity {
    #[serde(rename = "normal")]
    #[default]
    Normal,
    #[serde(rename = "legendary")]
    Legendary,
    #[serde(rename = "mythical")]
    Mythical,
}

impl PokemonRarity {
    pub fn get_color_for_embed(&self) -> Color {
        match self {
            PokemonRarity::Normal => Color::LIGHT_GREY,
            PokemonRarity::Legendary => Color::GOLD,
            PokemonRarity::Mythical => Color::BLITZ_BLUE,
        }
    }
}

// This is used by sqlx
impl From<String> for PokemonRarity {
    fn from(value: String) -> Self {
        match value.as_str() {
            "normal" => Self::Normal,
            "legendary" => Self::Legendary,
            "mythical" => Self::Mythical,
            _ => panic!("Failed to convert a string to a pokemon rarity"),
        }
    }
}

// This is used to read from the input file
impl FromStr for PokemonRarity {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "legendary" => Ok(Self::Legendary),
            "mythic" => Ok(Self::Mythical),
            _ => Err(format!("Got an invalid value while parsing a PokemonRarity: {}", s)),
        }
    }
}

// POKEMON
#[derive(Clone, Deserialize)]
pub struct Pokemon {
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub rarity: PokemonRarity,
    #[serde(default)]
    pub num: i64, // Pokedex entry can be negative on Showdown data for custom pokemon
    #[serde(default, rename(deserialize = "maxHP"))]
    pub max_hp: i64,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default, rename(deserialize = "heightm"))]
    pub height: f64,
    #[serde(default, rename(deserialize = "weightkg"))]
    pub weight: f64,
    #[serde(default)]
    pub color: String,
    #[serde(default, rename(deserialize = "canGigantamax"))]
    pub gigantamax: String,
    #[serde(default, rename(deserialize = "cannotDynamax"))]
    pub cannot_dynamax: bool,
    #[serde(default, rename(deserialize = "baseForme"))]
    pub base_form: String,
    #[serde(default, rename(deserialize = "baseSpecies"))]
    pub base_species: String,
    #[serde(default, rename(deserialize = "baseStats"))]
    pub base_stats: PokemonStats,
    #[serde(default, rename(deserialize = "changesFrom"))]
    pub changes_from: String,
    #[serde(default, rename(deserialize = "cosmeticFormes"))]
    pub cosmetic_forms: Vec<String>,
    #[serde(default, rename(deserialize = "otherFormes"))]
    pub other_forms: Vec<String>,
    #[serde(default, rename(deserialize = "isCosmeticForme"))]
    pub is_cosmetic_form: bool,
    #[serde(default, rename(deserialize = "formeOrder"))]
    pub form_order: Vec<String>,
    #[serde(default, rename(deserialize = "forme"))]
    pub form: String,
    #[serde(default, rename(deserialize = "evoCondition"))]
    pub evolution_condition: Option<String>,
    #[serde(default, rename(deserialize = "evoItem"))]
    pub evolution_item: Option<String>,
    #[serde(default, rename(deserialize = "evoLevel"))]
    pub evolution_level: Option<i64>,
    #[serde(default, rename(deserialize = "evoMove"))]
    pub evolution_move: Option<String>,
    #[serde(default, rename(deserialize = "evoRegion"))]
    pub evolution_region: Option<String>,
    #[serde(default, rename(deserialize = "evoType"))]
    pub evolution_type: Option<String>,
    #[serde(default, rename(deserialize = "evos"))]
    pub evolutions: Option<Vec<String>>,
    #[serde(default, rename(deserialize = "prevo"))]
    pub previous_evolution: Option<String>,
    #[serde(default)]
    pub types: Vec<String>, // TODO: This should be an enum
}
