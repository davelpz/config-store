# Config Store Microservice

A simple key-value store microservice built using the Rocket web framework, SQLite for storage, and `r2d2` for database connection pooling. This service allows storing, retrieving, updating, and deleting JSON data using keys. 

## Features
- **GET /<key>**: Retrieve the value associated with a key. 
- **POST /<key>**: Store JSON data under a key. 
- **PUT /<key>**: Update the data under a key. 
- **DELETE /<key>**: Remove a key and its associated value. 
- **HEAD /<key>**: Check if a key exists without retrieving the value. 
- **GET /keys**: List all keys stored in the database. 
- **GET /count**: Count the number of stored keys. 
 
## Installation
1. **Clone the repository:**
```bash
git clone https://github.com/davelpz/config-store.git 
cd config-store
```

2. **Build the project:**
```bash
cargo build --release
``` 

3. **Run the service:**
```bash
cargo run --release
```
The service will start on http://localhost:8000




