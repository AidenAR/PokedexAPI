# PokeDex API

RESTful API for managing Pokémon using [Axum](https://github.com/tokio-rs/axum). 

It supportsCRUD operations as well as fetching data from the [PokéAPI](https://pokeapi.co/).

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

GET /fetch/:name
Fetches a single Pokémon from the PokéAPI and stores it in the database.
