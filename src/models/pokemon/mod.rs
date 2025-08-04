use std::collections::HashMap;
use std::fs;

use rand::{Rng, seq::IndexedRandom};
use serde_json::Value;
use serenity::prelude::TypeMapKey;

pub mod pokemon_command_record;
pub mod pokemon_ownership_record;

pub fn roll_for_shiny() -> bool {
    let mut rng = rand::rng();
    let rand_num = rng.random_range(0..100);
    rand_num > 94
}

pub fn roll_for_april_fools() -> bool {
    let mut rng = rand::rng();
    let rand_num = rng.random_range(0..100);
    rand_num > 79
}

pub struct PokemonData {
    data: HashMap<String, Vec<String>>,
}

impl PokemonData {
    pub fn new() -> Self {
        let mut data: HashMap<String, Vec<String>> = HashMap::new();

        let contents =
            fs::read_to_string("pokemon_data.json").expect("Failed to read pokemon_data.json file");
        let v: Value =
            serde_json::from_str(&contents).expect("Failed to parse pokemon_data.json from JSON");

        match v {
            Value::Object(pokemon_map) => {
                let pokemon_list_entry = pokemon_map
                    .get("ani")
                    .expect("Pokemon map must have an 'ani' key");
                let pokemon_list = PokemonData::read_pokemon_list(pokemon_list_entry);
                data.insert("ani".to_owned(), pokemon_list);

                let april_fools_list_entry = pokemon_map
                    .get("april_fools")
                    .expect("Pokemon map must have an 'april_fools' key");
                let april_fools_list = PokemonData::read_pokemon_list(april_fools_list_entry);
                data.insert("afd".to_owned(), april_fools_list);
            }
            _ => panic!("Pokemon data must open with a Map"),
        }

        return Self { data };
    }

    fn read_pokemon_list(entry: &Value) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        match entry {
            Value::Array(pokemon_list) => {
                for entry in pokemon_list {
                    match entry {
                        Value::String(pokemon_name) => list.push(pokemon_name.to_owned()),
                        _ => panic!("Pokemon map entry list items must all be strings"),
                    }
                }
            }
            _ => panic!("Pokemon map entry must be a list"),
        }
        list
    }

    fn get_random_entry(&self, map_key: &str) -> Option<String> {
        let pokemon_list = match self.data.get(map_key) {
            Some(list) => list,
            None => return None,
        };
        let random_entry = pokemon_list
            .choose(&mut rand::rng())
            .expect("Pokemon map entry must contain entries");
        Some(random_entry.to_owned())
    }

    pub fn get_random_pokemon(&self) -> String {
        self.get_random_entry("afd")
            .expect("Pokemon map must contain an 'afd' key")
    }

    pub fn get_random_april_fools_pokemon(&self) -> String {
        self.get_random_entry("ani")
            .expect("Pokemon map must contain an 'ani' key")
    }
}

impl TypeMapKey for PokemonData {
    type Value = PokemonData;
}
