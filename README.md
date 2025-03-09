# PokeDex API

A RESTful API in RUST for managing Pokémon using [Axum](https://github.com/tokio-rs/axum). 

supports basic CRUD operations as well as fetching and storing data from the [PokéAPI](https://pokeapi.co/). 

All Pokémon data is persisted in a local JSON file, `pokemon_db.json` for now. -> the json file right now is populated with some tests I ran (see pics below)

## Features
- **Create, Read, Update, Delete** Pokémon in a file-based “database.”
- **Fetch** Pokémon data from the PokéAPI, store it locally, and retrieve it through the API.
- **Concurrent** fetching of multiple Pokémon at once using Tokio’s async runtime.

## Requirements
- [Rust](https://www.rust-lang.org/tools/install) (stable or latest)
- [Cargo](https://doc.rust-lang.org/cargo/) (bundled with Rust)
- Internet access to fetch data from the PokéAPI

## Installation & Running
1. **Clone** this repository or copy the source code.
2. **Navigate** into the project directory:
   ```bash
   cd pokemon_api


## Endpoints

### CRUD Endpoints

#### POST `/pokemon`

Creates a new Pokémon with a generated UUID.

**Request:**

```
curl -X POST -H "Content-Type: application/json" \
-d '{"name": "Pikachu", "type_": "Electric", "level": 5, "hp": 35}' \
http://localhost:3000/pokemon
```

#### GET /pokemon/:id
Retrieves the Pokémon with the specified UUID.

```
curl http://localhost:3000/pokemon/6b9e61f9-b4cf-4552-8260-79ff83e5ff88
Response:
```

### PUT /pokemon/:id
Updates the details of an existing Pokémon.

```
curl -X PUT -H "Content-Type: application/json" \
-d '{"id": "6b9e61f9-b4cf-4552-8260-79ff83e5ff88", "name": "Raichu", "type_": "Electric", "level": 10, "hp": 60}' \
http://localhost:3000/pokemon/6b9e61f9-b4cf-4552-8260-79ff83e5ff88
```

## DELETE /pokemon/:id
Removes the Pokémon from the database.

```
curl -X DELETE http://localhost:3000/pokemon/6b9e61f9-b4cf-4552-8260-79ff83e5ff88
```

### PokéAPI Endpoints

## GET /fetch/:name
Fetches a single Pokémon from the PokéAPI by name, stores it in the local database, and returns it.

```
curl http://localhost:3000/fetch/ditto
```


## GET /fetch_all?names=<comma-separated-names>

Fetches multiple Pokémon concurrently from the PokéAPI by listing their names (e.g., ditto,pikachu,bulbasaur). 
Each successfully fetched Pokémon is stored in the database.

```
curl "http://localhost:3000/fetch_all?names=ditto,pikachu,bulbasaur"
```


### Concurrency Notes
The app uses Tokio to run async tasks, allowing multiple I/O-bound operations to happen concurrently

When calling /fetch_all, each Pokémon fetch is spawned in a separate task, and results are awaited together.


### Happy Pokémon Catching!



## EXAMPLES:

![Screenshot 2025-03-09 at 1 41 33 PM](https://github.com/user-attachments/assets/311da5b3-9c13-4bf6-a820-53b305bca476)

![Screenshot 2025-03-09 at 12 30 55 PM](https://github.com/user-attachments/assets/7dca5acd-61e2-4ce2-b225-4487079a6f72)

![Screenshot 2025-03-09 at 12 26 20 PM](https://github.com/user-attachments/assets/00234f94-b0a8-441c-97b9-091ca8e66cbf)

![Screenshot 2025-03-09 at 12 25 48 PM](https://github.com/user-attachments/assets/bfc1ba5d-6042-43b0-bcd6-c1c87443d0b8)

![Screenshot 2025-03-09 at 12 11 50 PM](https://github.com/user-attachments/assets/2ff64eaf-e1e6-4f3f-bfcf-2c6edf51560b)

![Screenshot 2025-03-09 at 12 11 29 PM](https://github.com/user-attachments/assets/51423549-9b6d-4a89-920c-1bb2bec98994)







