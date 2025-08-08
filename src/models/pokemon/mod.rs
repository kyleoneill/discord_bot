use std::fs;
use std::{collections::HashMap, str::FromStr};

use rand::{Rng, seq::IndexedRandom};
use serde_json::Value;
use serenity::prelude::TypeMapKey;

pub mod pokemon_command_record;
pub mod pokemon_ownership_record;
use pokemon_ownership_record::{Pokemon, PokemonType};

use crate::models::pokemon::pokemon_ownership_record::PokemonClassification;

pub fn roll_for_type() -> PokemonType {
    let mut rng = rand::rng();
    let rand_num = rng.random_range(0..100);
    if rand_num > 94 {
        return PokemonType::Shiny;
    }
    if rand_num > 74 {
        return PokemonType::AprilFools;
    }
    return PokemonType::Normal;
}

pub fn pokemon_link_to_name(link: &str) -> String {
    let intermediate = link[0..(link.len() - 4)].replace('-', " ");
    let mut split = intermediate.split_whitespace();
    let mut build_string = String::new();
    while let Some(next) = split.next() {
        let mut chars = next.chars();
        let capitalized_str = match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        };
        build_string.push_str(" ");
        build_string.push_str(capitalized_str.as_str());
    }
    build_string
}

pub struct PokemonData {
    data: HashMap<String, Vec<Pokemon>>,
}

impl PokemonData {
    pub fn new() -> Self {
        let mut data: HashMap<String, Vec<Pokemon>> = HashMap::new();

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

    fn read_pokemon_list(entry: &Value) -> Vec<Pokemon> {
        let mut list: Vec<Pokemon> = Vec::new();
        match entry {
            Value::Array(pokemon_list) => {
                for entry in pokemon_list {
                    match entry {
                        Value::Object(pokemon_object) => {
                            let name = {
                                let val = pokemon_object
                                    .get("name")
                                    .expect("Pokemon object must have a 'name' key");
                                match val {
                                    Value::String(s) => s.to_owned(),
                                    _ => {
                                        panic!("Pokemon object 'name' key must have a string value")
                                    }
                                }
                            };
                            let classification = {
                                let val = pokemon_object
                                    .get("classification")
                                    .expect("Pokemon object must have a 'classification' key");
                                match val {
                                    Value::String(s) => PokemonClassification::from_str(s).expect("PokemonClassification must be valid in the static data file"),
                                    _ => panic!("Pokemon object 'classification' key must have a string value")
                                }
                            };
                            list.push(Pokemon {
                                name,
                                classification,
                            });
                        }
                        _ => panic!("Pokemon map entry list items must all be objects"),
                    }
                }
            }
            _ => panic!("Pokemon map entry must be a list"),
        }
        list
    }

    fn get_random_entry(&self, map_key: &str) -> Option<Pokemon> {
        let pokemon_list = match self.data.get(map_key) {
            Some(list) => list,
            None => return None,
        };
        let random_entry = pokemon_list
            .choose(&mut rand::rng())
            .expect("Pokemon map entry must contain entries");
        Some(random_entry.clone())
    }

    pub fn get_random_pokemon(&self, pokemon_type: &PokemonType) -> Pokemon {
        match pokemon_type {
            PokemonType::AprilFools => self
                .get_random_entry("afd")
                .expect("Pokemon map must contain an 'afd' key"),
            _ => self
                .get_random_entry("ani")
                .expect("Pokemon map must contain an 'ani' key"),
        }
    }
}

impl TypeMapKey for PokemonData {
    type Value = PokemonData;
}
