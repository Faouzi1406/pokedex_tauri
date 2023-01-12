mod pokemon_structs;
mod spawn_window;
mod pokemon_types;
mod pokemon_abilities;

use uuid::Uuid;
use pokemon_rs::*;
use spawn_window::*;

use pokemon_structs::{
    AllPokemon,
    Pokemon,
    get_pokemon
};

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn unique_id() -> String {
    let id = Uuid::new_v4();

    format!("{:?}", id)
}


#[tauri::command]
fn get_all_pokemon() -> AllPokemon {
    let pokemon_get = pokemon_rs::get_all(None);
    let mut get_all_pokemon:AllPokemon  =AllPokemon::new();

    for pokemon in pokemon_get{
        let id  = pokemon_rs::get_id_by_name(pokemon, None);
        let pokemon:Pokemon = Pokemon { 
            id,
            name: pokemon.to_owned(), 
            sound: format!("{}.mp3", pokemon), 
            image: format!("{}.jpg", pokemon) 
        };

        get_all_pokemon.all_pokemon.push(pokemon)
    }

   get_all_pokemon 
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            spawn_window,
            get_all_pokemon,
            unique_id, 
            get_pokemon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
