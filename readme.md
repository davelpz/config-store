# Config Store Microservice

A simple key-value store microservice built using the Rocket web framework, SQLite for storage, and `r2d2` for database connection pooling. This service allows storing, retrieving, updating, and deleting JSON data using keys. 

## Features
- **GET /config/\<env>/\<app>**: Retrieve the config data for a given environment and application.  
- **POST /config/\<env>/\<app>**: Store JSON config data for a given environment and application.
- **PUT /config/\<env>/\<app>**: Update the config data for a given environment and application.
- **DELETE /config/\<env>/\<app>**: Remove the config data for a given environment and application.
- **HEAD /config/\<env>/\<app>**: Check if a config exists for a given environment and application.
- **GET /keys**: List all the config keys stored in the database.
- **GET /count**: Count the number of config keys stored in the database.
 
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

## API Usage
1. **Get Data**
Retrieve the JSON config data for a given environment and application.
```bash
curl http://localhost:8000/config/<env>/<app>
```
2. **Store Data(POST)**
Store JSON config data for a given environment and application.
```bash
curl -X POST -H "Content-Type: application/json" -d '{"key": "value"}' http://localhost:8000/config/<env>/<app>
```
3. **Update Data(PUT)**
Update the JSON config data for a given environment and application.
```bash
curl -X PUT -H "Content-Type: application/json" -d '{"key": "value"}' http://localhost:8000/config/<env>/<app>
```
4. **Delete Data**
Remove the JSON config data for a given environment and application.
```bash
curl -X DELETE http://localhost:8000/config/<env>/<app>
```
5. **Check if Key Exists**
Check if a config exists for a given environment and application.
```bash
curl -I http://localhost:8000/config/<env>/<app>
```
6. **List All Keys**
Retrieve a list of all keys in the database.
```bash
curl http://localhost:8000/keys
```
7. **Count Keys**
Retrieve the total number of keys stored in the database.
```bash
curl http://localhost:8000/count
```

## Configuration
- The service uses an SQLite database file named kv.db.
- The database connection pool is managed by r2d2 with a maximum pool size of 15 connections.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
Contributions are welcome! Please feel free to submit a pull request if
you have improvements or features you would like to see in this project.

## Acknowledgements
- [Rocket](https://rocket.rs/)
- [rusqlite](https://github.com/rusqlite/rusqlite)
- [SQLite](https://www.sqlite.org/index.html)
- [r2d2](https://github.com/sfackler/r2d2)
- [Serde](https://serde.rs/)
