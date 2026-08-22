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

        let contents = fs::read_to_string("data/pokedex.json").expect("Failed to read pokedex.json file");
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

        const ATTEMPTS: usize = 5;
        // Try to get a pokemon a few times
        for _ in 0..ATTEMPTS {
            let index = rng.random_range(0..self.data.len());
            let mut pokemon = self.data.get_index(index).expect("Failed to get a random pokemon").1.clone();

            // Apply cosmetic forms. This must be done before the pokedex number
            // check because cosmetic forms don't have their pokedex number in
            // their entry.
            if pokemon.is_cosmetic_form {
                let cosmetic_form = pokemon;
                pokemon = self
                    .data
                    .get(&cosmetic_form.base_species.to_lowercase())
                    .expect("Failed to get base form of cosmetic form")
                    .clone();
                pokemon.is_cosmetic_form = true;
                pokemon.name = cosmetic_form.name;
                pokemon.base_species = cosmetic_form.base_species;
                pokemon.form = cosmetic_form.form;
                pokemon.color = cosmetic_form.color;
            }

            if pokemon.num > 0 {
                return pokemon;
            }

            // We rolled missingno or an unofficial Create-A-Pokemon project
            // Pokemon, try again
        }

        // Failed to get a pokemon, return missingno as an easter egg
        self.data.get("missingno").expect("no missingno in pokedex").clone()
    }
}

impl TypeMapKey for PokemonData {
    type Value = PokemonData;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemon_data_schema() {
        // Check to make sure that the pokemon data matches the expected schema
        let _ = PokemonData::new();
    }
}
