use serde::{Deserialize, Serialize};
use uuid::Uuid;

//Our internal Pokémon model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pokemon {
    pub id: Uuid,
    pub name: String,
    pub type_: String,
    pub level: u8,
    pub hp: u16,
}

//Represents data provided by the client when creating a new Pokémon
#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonInput {
    pub name: String,
    pub type_: String,
    pub level: u8,
    pub hp: u16,
}

//  PokéAPI response model
#[derive(Deserialize)]
pub struct PokeAPIResponse {
    pub name: String,
    pub types: Vec<PokeType>,
}

#[derive(Deserialize)]
pub struct PokeType {
    #[serde(rename = "type")]
    pub poke_type: TypeName,
}

#[derive(Deserialize)]
pub struct TypeName {
    pub name: String,
}
