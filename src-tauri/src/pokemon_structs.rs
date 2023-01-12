use crate::{
    pokemon_types::*,
    pokemon_abilities::*
};

pub enum PokemonNotFound {
    IDOUTOFSCOPE
}

#[derive(Debug, serde::Serialize)]
pub struct Pokemon {
    pub id:usize,
    pub name: String, 
    pub sound: String, 
    pub image: String 
}

#[derive(Debug, serde::Serialize)]
pub struct AllPokemon {
    pub all_pokemon: Vec<Pokemon>
}


impl AllPokemon {
    pub fn new() -> Self{
        AllPokemon { 
            all_pokemon: Vec::new()
        }
    }
}

pub struct PokemonStats {
    hp:usize,
    attack:usize,
    defense:usize,
    sp_attack:usize,
    sp_def:usize,
    speed:usize,
    total:usize
}

/// Informatie over pokemon denk hier aan Type, height etc..
pub struct PokemonInformation {
    type_pokemon:Vec<PokemonType>,
    height:f32,
    weight:f32,
    abilities:Vec<PokemonAbilities>,
}

pub struct PokemonDetails {
    pokemon:Pokemon,
    pokemon_stats: PokemonStats,
    pokemon_information:PokemonInformation
}

impl PokemonDetails {
    fn new(pokemon_id:usize) /* -> Result<Self, PokemonNotFound> */ {
        //Check of pokemon id in scope is tussen 1 en 900

        // if pokemon_id <1 || pokemon_id >900 {
        //     return Err(PokemonNotFound::IDOUTOFSCOPE);
        // }

        let pokemon_name = pokemon_rs::get_by_id(pokemon_id, None).to_lowercase();
        let query_url  = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);

        let pokemon_get = reqwest::blocking::get(query_url)
            .unwrap()
            .text()
            .unwrap();

        println!("{pokemon_get}");
    }
}


#[tauri::command]
pub fn get_pokemon(pokemon_id:usize) ->String{
    PokemonDetails::new(pokemon_id);
    "worked".to_string()
}
