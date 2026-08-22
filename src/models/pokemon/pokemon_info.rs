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
    /*
        link generation logic in pokemon showdown code is in `getSpriteData` in `src/battle-dex.ts`, as of when this was written
    */
    pub fn get_link_for_type(&self, pokemon: &Pokemon) -> String {
        let pokemon_link_name = pokemon.generate_showdown_link_name();
        let path = match self {
            Self::Normal => {
                // Some pokemon are missing animated sprites. Fall back to the
                // pokemon home static sprites for those cases.
                if pokemon.has_ani_sprite {
                    format!("ani/{pokemon_link_name}.gif")
                } else {
                    format!("home-centered/{pokemon_link_name}.png")
                }
            }
            Self::AprilFools => format!("afd/{pokemon_link_name}.png"),
            Self::Shiny => {
                // Some pokemon are missing animated sprites. Fall back to the
                // pokemon home static sprites for those cases.
                if pokemon.has_ani_sprite {
                    format!("ani-shiny/{pokemon_link_name}.gif")
                } else {
                    format!("home-centered-shiny/{pokemon_link_name}.png")
                }
            }
        };
        format!("https://play.pokemonshowdown.com/sprites/{path}")
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
#[derive(Clone, Default, Deserialize)]
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
    #[serde(default = "has_ani_sprite_default")]
    pub has_ani_sprite: bool,
}

fn has_ani_sprite_default() -> bool {
    true
}

impl Pokemon {
    pub fn generate_showdown_link_name(&self) -> String {
        fn to_id(s: &str) -> String {
            s.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect()
        }

        let base_species = if !self.base_species.is_empty() { &self.base_species } else { &self.name };
        let mut sprite_id = to_id(base_species);
        if !self.base_species.is_empty() {
            sprite_id = format!("{sprite_id}-{}", to_id(&self.form));
        };

        match sprite_id.as_str() {
            "greninja-bond" => "greninja".to_string(),
            "rockruff-dusk" => "rockruff".to_string(),
            _ => sprite_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_name_simple() {
        let bulbasaur = Pokemon {
            // base_species
            name: "Bulbasaur".to_string(),
            // form
            ..Default::default()
        };
        assert_eq!(bulbasaur.generate_showdown_link_name(), "bulbasaur");
    }

    #[test]
    fn test_link_name_non_ascii() {
        let flabebe = Pokemon {
            // base_species
            name: "Flabe\u{0301}be\u{0301}".to_string(),
            // form
            ..Default::default()
        };
        assert_eq!(flabebe.generate_showdown_link_name(), "flabebe");
    }

    #[test]
    fn test_link_name_cosmetic_forme() {
        let vivillon_icy_snow = Pokemon {
            base_species: "Vivillon".to_string(),
            name: "Vivillon-Icy Snow".to_string(),
            form: "Icy Snow".to_string(),
            ..Default::default()
        };
        assert_eq!(vivillon_icy_snow.generate_showdown_link_name(), "vivillon-icysnow");
    }

    #[test]
    fn test_link_name_forme_non_ascii() {
        let dudunsparce_three_segment = Pokemon {
            base_species: "Dudunsparce".to_string(),
            name: "Dudunsparce-Three-Segment".to_string(),
            form: "Three-Segment".to_string(),
            ..Default::default()
        };
        assert_eq!(dudunsparce_three_segment.generate_showdown_link_name(), "dudunsparce-threesegment");
    }

    #[test]
    fn test_link_name_non_ascii_forme() {
        let farfetchd_galar = Pokemon {
            base_species: "Farfetch\u{2019}d".to_string(),
            name: "Farfetch\u{2019}d-Galar".to_string(),
            form: "Galar".to_string(),
            ..Default::default()
        };
        assert_eq!(farfetchd_galar.generate_showdown_link_name(), "farfetchd-galar");
    }

    #[test]
    fn test_link_name_mega() {
        let glalie_mega = Pokemon {
            base_species: "Glalie".to_string(),
            name: "Glalie-Mega".to_string(),
            form: "Mega".to_string(),
            ..Default::default()
        };
        assert_eq!(glalie_mega.generate_showdown_link_name(), "glalie-mega");
    }

    #[test]
    fn test_link_name_gmax() {
        let centiskorch_gmax = Pokemon {
            base_species: "Centiskorch".to_string(),
            name: "Centiskorch-Gmax".to_string(),
            form: "Gmax".to_string(),
            ..Default::default()
        };
        assert_eq!(centiskorch_gmax.generate_showdown_link_name(), "centiskorch-gmax");
    }

    #[test]
    fn test_link_name_totem() {
        let gumshoos_totem = Pokemon {
            base_species: "Gumshoos".to_string(),
            name: "Gumshoos-Totem".to_string(),
            form: "Totem".to_string(),
            ..Default::default()
        };
        assert_eq!(gumshoos_totem.generate_showdown_link_name(), "gumshoos-totem");
    }

    #[test]
    fn test_link_name_primal() {
        let kyogre_primal = Pokemon {
            base_species: "Kyogre".to_string(),
            name: "Kyogre-Primal".to_string(),
            form: "Primal".to_string(),
            ..Default::default()
        };
        assert_eq!(kyogre_primal.generate_showdown_link_name(), "kyogre-primal");
    }

    #[test]
    fn test_link_name_explicit_exceptions() {
        let greninja_bond = Pokemon {
            base_species: "Greninja".to_string(),
            name: "Greninja-Bond".to_string(),
            form: "Bond".to_string(),
            ..Default::default()
        };
        assert_eq!(greninja_bond.generate_showdown_link_name(), "greninja");

        let rockruff_dusk = Pokemon {
            base_species: "RockRuff".to_string(),
            name: "Rockruff-Dusk".to_string(),
            form: "Dusk".to_string(),
            ..Default::default()
        };
        assert_eq!(rockruff_dusk.generate_showdown_link_name(), "rockruff");
    }

    #[test]
    fn test_ani_sprite() {
        let venusaur_mega = Pokemon {
            base_species: "Venusaur".to_string(),
            name: "Venusaur-Mega".to_string(),
            form: "Mega".to_string(),
            has_ani_sprite: true,
            ..Default::default()
        };
        assert_eq!(
            PokemonType::Normal.get_link_for_type(&venusaur_mega),
            "https://play.pokemonshowdown.com/sprites/ani/venusaur-mega.gif",
        );
        assert_eq!(
            PokemonType::Shiny.get_link_for_type(&venusaur_mega),
            "https://play.pokemonshowdown.com/sprites/ani-shiny/venusaur-mega.gif",
        );
        assert_eq!(
            PokemonType::AprilFools.get_link_for_type(&venusaur_mega),
            "https://play.pokemonshowdown.com/sprites/afd/venusaur-mega.png",
        );
    }

    #[test]
    fn test_missing_ani_sprite() {
        let venusaur_gmax = Pokemon {
            base_species: "Venusaur".to_string(),
            name: "Venusaur-Gmax".to_string(),
            form: "Gmax".to_string(),
            has_ani_sprite: false,
            ..Default::default()
        };
        assert_eq!(
            PokemonType::Normal.get_link_for_type(&venusaur_gmax),
            "https://play.pokemonshowdown.com/sprites/home-centered/venusaur-gmax.png",
        );
        assert_eq!(
            PokemonType::Shiny.get_link_for_type(&venusaur_gmax),
            "https://play.pokemonshowdown.com/sprites/home-centered-shiny/venusaur-gmax.png",
        );
        assert_eq!(
            PokemonType::AprilFools.get_link_for_type(&venusaur_gmax),
            "https://play.pokemonshowdown.com/sprites/afd/venusaur-gmax.png",
        );
    }
}
