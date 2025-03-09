use std::collections::HashMap;
use std::fs;
use uuid::Uuid;
use crate::models::Pokemon;

//Loads the database from "pokemon_db.json" ->  returns an empty HashMap if error
pub fn load_db() -> HashMap<Uuid, Pokemon> {
    if let Ok(contents) = fs::read_to_string("pokemon_db.json") {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

// Saves the current database (a HashMap) to "pokemon_db.json" -> we could use a sql server or mongo db instead
pub fn save_db(db: &HashMap<Uuid, Pokemon>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(db)?;
    fs::write("pokemon_db.json", json)
}
