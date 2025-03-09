mod db;
mod models;
mod routes;

use std::sync::{Arc, Mutex};
use crate::db::load_db;
use crate::routes::{create_router, PokemonDb};
use axum::Server;

#[tokio::main]
async fn main() {
    // Load or create the database
    let initial_db = load_db();
    let db: PokemonDb = Arc::new(Mutex::new(initial_db));

    // Build our application router
    let app = create_router(db);

    // Run the Axum server
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
