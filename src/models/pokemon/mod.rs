use std::fs;

use indexmap::IndexMap;
use rand::Rng;
use serde_json::Value;
use serenity::prelude::TypeMapKey;

pub mod pokemon_command_record;
pub mod pokemon_info;
pub mod pokemon_ownership_record;

use pokemon_info::{Pokemon, PokemonType};

pub fn roll_for_type() -> PokemonType {
    // TODO: Define these chances outside of this function somewhere, magic numbers like this are not good
    let mut rng = rand::rng();
    let rand_num = rng.random_range(0..100);
    if rand_num > 94 {
        return PokemonType::Shiny;
    }
    if rand_num > 74 {
        return PokemonType::AprilFools;
    }
    PokemonType::Normal
}

pub struct PokemonData {
    data: IndexMap<String, Pokemon>,
}

#[allow(clippy::new_without_default)]
impl PokemonData {
    pub fn new() -> Self {
        let mut data: IndexMap<String, Pokemon> = IndexMap::new();

        let contents = fs::read_to_string("data/pokedex_gold.json").expect("Failed to read pokemon_data.json file");
        let v: Value = serde_json::from_str(&contents).expect("Failed to parse pokemon JSON");

        match v {
            Value::Object(pokemon_map) => {
                for key in pokemon_map.keys() {
                    let entry = pokemon_map.get(key).expect("Failed to get a pokemon with a known key");
                    let mut parsed_pokemon: Pokemon = serde_json::from_value(entry.to_owned()).expect("Failed to parse pokemon entry from JSON");
                    // This is a hack because of limitations in my data scraper which generates the pokemon list
                    parsed_pokemon.slug = key.to_owned();
                    data.insert(key.to_owned(), parsed_pokemon);
                }
            }
            _ => panic!("Pokemon data must open with an Object"),
        }

        Self { data }
    }

    pub fn get_random_pokemon(&self) -> Pokemon {
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.data.len());
        let (_pokemon_name, pokemon) = self.data.get_index(index).expect("Failed to get a random pokemon");
        pokemon.clone()
    }
}

impl TypeMapKey for PokemonData {
    type Value = PokemonData;
}
