use crate::{
    pokemon_types::*,
    pokemon_abilities::*
};

#[derive(Debug)]
pub enum PokemonNotFound {
    IDOUTOFSCOPE
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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


/// Informatie over pokemon denk hier aan Type, height etc..
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PokemonInformation {
    //type_pokemon:Vec<PokemonType>,
    height:f32,
    weight:f32,
    base_experience:Option<usize>,
    abilities:Vec<Abilities>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PokemonDetails {
    pokemon:Pokemon,
    pokemon_information:PokemonInformation
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Ability {
    name: String,
    url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Abilities {
    ability: Ability,
    is_hidden: bool,
    slot: i32,
}


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PokemonFetch {
    name:String,
    base_experience:Option<usize>,
    height:f32,
    weight:f32,
    abilities:Vec<Abilities>
}

impl PokemonDetails {
    fn new(pokemon_id:usize) -> Result<Self, PokemonNotFound> {

        if pokemon_id <1 || pokemon_id >900 {
             return Err(PokemonNotFound::IDOUTOFSCOPE);
        }

        let pokemon_name = pokemon_rs::get_by_id(pokemon_id, None).to_lowercase();
        let query_url  = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);

        let pokemon_get:PokemonFetch = reqwest::blocking::get(query_url)
            .unwrap()
            .json()
            .unwrap();
        print!("{:#?}", pokemon_get);

        let pokemon:Pokemon = Pokemon {
            id:pokemon_id,
            name:pokemon_get.name.to_string(),
            sound:format!("{pokemon_name}.mp3"),
            image:format!("{pokemon_name}.mp3")
        };

        let pokemon_information:PokemonInformation = PokemonInformation {
            height: pokemon_get.height,
            weight: pokemon_get.weight,
            base_experience: pokemon_get.base_experience,
            abilities:pokemon_get.abilities
        };

        return Ok(PokemonDetails {
                pokemon,
                pokemon_information
        });
    }
}


#[tauri::command]
pub fn get_pokemon(pokemon_id:usize) -> PokemonDetails {
    PokemonDetails::new(pokemon_id).unwrap()
}
