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
