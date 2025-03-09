use axum::{
    extract::{Path, State, Json, Query},
    http::StatusCode,
    Router,
    routing::{get, post, put, delete},
};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;
use futures::future::join_all;
use reqwest;
use serde::Deserialize;
use crate::{
    db::{load_db, save_db},
    models::{Pokemon, PokemonInput, PokeAPIResponse, PokeType, TypeName},
};

// Shared database type
pub type PokemonDb = Arc<Mutex<HashMap<Uuid, Pokemon>>>;

// CRUD Handlers

//GET /pokemon/:id
pub async fn get_pokemon(
    State(db): State<PokemonDb>,
    Path(id): Path<Uuid>
) -> Result<Json<Pokemon>, StatusCode> {
    let db = db.lock().unwrap();
    db.get(&id)
        .map(|pokemon| Json(pokemon.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /pokemon
pub async fn add_pokemon(
    State(db): State<PokemonDb>,
    Json(input): Json<PokemonInput>
) -> Result<(StatusCode, Json<Pokemon>), StatusCode> {
    let mut db = db.lock().unwrap();
    let new_pokemon = Pokemon {
        id: Uuid::new_v4(),
        name: input.name,
        type_: input.type_,
        level: input.level,
        hp: input.hp,
    };
    db.insert(new_pokemon.id, new_pokemon.clone());
    if let Err(e) = save_db(&db) {
        eprintln!("Error saving DB: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok((StatusCode::CREATED, Json(new_pokemon)))
}

// PUT /pokemon/:id
pub async fn update_pokemon(
    State(db): State<PokemonDb>,
    Path(id): Path<Uuid>,
    Json(pokemon): Json<Pokemon>
) -> StatusCode {
    let mut db = db.lock().unwrap();
    if db.contains_key(&id) {
        db.insert(id, pokemon);
        if let Err(e) = save_db(&db) {
            eprintln!("Error saving DB: {:?}", e);
        }
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

// DELETE /pokemon/:id
pub async fn delete_pokemon(
    State(db): State<PokemonDb>,
    Path(id): Path<Uuid>
) -> StatusCode {
    let mut db = db.lock().unwrap();
    if db.remove(&id).is_some() {
        if let Err(e) = save_db(&db) {
            eprintln!("Error saving DB: {:?}", e);
        }
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// PokéAPI Handlers

/// Fetch a single Pokémon from the PokéAPI and convert it to our local Pokemon 
pub async fn fetch_pokemon_data(url: &str) -> Result<Pokemon, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let api_pokemon: PokeAPIResponse = response.json().await?;
    let poke_type = api_pokemon
        .types
        .get(0)
        .map_or("unknown".to_string(), |pt| pt.poke_type.name.clone());

    Ok(Pokemon {
        id: Uuid::new_v4(),
        name: api_pokemon.name,
        type_: poke_type,
        level: 1,
        hp: 100,
    })
}

//  Fetch multiple Pokémon concurrently from a list of URLs
pub async fn fetch_multiple_pokemon(urls: Vec<String>) -> Vec<Result<Pokemon, reqwest::Error>> {
    let client = reqwest::Client::new();
    let futures = urls.into_iter().map(|url| {
        let client = client.clone();
        tokio::spawn(async move {
            let response = client.get(&url).send().await?;
            let api_pokemon: PokeAPIResponse = response.json().await?;
            let poke_type = api_pokemon
                .types
                .get(0)
                .map_or("unknown".to_string(), |pt| pt.poke_type.name.clone());
            Ok(Pokemon {
                id: Uuid::new_v4(),
                name: api_pokemon.name,
                type_: poke_type,
                level: 1,
                hp: 100,
            })
        })
    }).collect::<Vec<_>>();

    // Join all spawned tasks and collect their results
    let results = join_all(futures).await;
    results.into_iter().map(|join_handle_res| join_handle_res.unwrap()).collect()
}

//GET /fetch/:name
pub async fn fetch_and_store_pokemon(
    State(db): State<PokemonDb>,
    Path(name): Path<String>,
) -> Result<(StatusCode, Json<Pokemon>), StatusCode> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
    match fetch_pokemon_data(&url).await {
        Ok(pokemon) => {
            let mut db = db.lock().unwrap();
            db.insert(pokemon.id, pokemon.clone());
            if let Err(e) = save_db(&db) {
                eprintln!("Error saving DB: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok((StatusCode::CREATED, Json(pokemon)))
        },
        Err(e) => {
            eprintln!("Error fetching Pokémon from API: {:?}", e);
            Err(StatusCode::BAD_GATEWAY)
        }
    }
}

//Query parameters for /fetch_all?names=ditto,pikachu,bulbasaur
#[derive(Deserialize)]
pub struct FetchAllQuery {
    pub names: String,
}

//  GET /fetch_all?names=ditto,pikachu,bulbasaur
use axum::response::IntoResponse;


//Fetch multiple Pokémon from the PokéAPI concurrently using `fetch_multiple_pokemon`,
//   store them in the DB, and return them in one response.
pub async fn fetch_all_pokemon(
    State(db): State<PokemonDb>,
    Query(query): Query<FetchAllQuery>,
) -> Result<(StatusCode, Json<Vec<Pokemon>>), (StatusCode, String)> {
     // Split the comma-separated names into a vector of strings.
    let names: Vec<&str> = query
        .names
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    // Build URLs like https://pokeapi.co/api/v2/pokemon/ditto
    let urls: Vec<String> = names
        .iter()
        .map(|name| format!("https://pokeapi.co/api/v2/pokemon/{}", name))
        .collect();

    // Fetch the Pokémon concurrently
    let results = fetch_multiple_pokemon(urls).await;

    // Lock the DB for insertion
    let mut db_handle = db.lock().unwrap();
    let mut stored_pokemon = Vec::new();

     //Only add Pokémons that were fetched successfully
    for result in results {
        if let Ok(pokemon) = result {
            db_handle.insert(pokemon.id, pokemon.clone());
            stored_pokemon.push(pokemon);
        } else if let Err(e) = result {
            eprintln!("Failed to fetch one Pokémon: {:?}", e);
        }
    }

     // Save the updated DB
    if let Err(e) = save_db(&db_handle) {
        eprintln!("Error saving DB: {:?}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error saving DB".into()));
    }

    Ok((StatusCode::CREATED, Json(stored_pokemon)))
}


//Create and return the Axum router with all routes
pub fn create_router(db: PokemonDb) -> Router {
    Router::new()
        // CRUD
        .route("/pokemon/:id", get(get_pokemon))
        .route("/pokemon", post(add_pokemon))
        .route("/pokemon/:id", put(update_pokemon))
        .route("/pokemon/:id", delete(delete_pokemon))
        // Single fetch
        .route("/fetch/:name", get(fetch_and_store_pokemon))
        // Multiple fetch
        .route("/fetch_all", get(fetch_all_pokemon))
        .with_state(db)
}
