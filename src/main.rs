mod db;
use db::{create_schema, get_data, put_data};
use rocket::response::content;
use rocket::State;
use rocket::serde::json::Json;
use r2d2::{Pool};
use r2d2_sqlite::SqliteConnectionManager;

#[macro_use] extern crate rocket;

type DbPool = Pool<SqliteConnectionManager>;

/// Get data from a key.
#[get("/<key>")]
fn get(key: &str, conn: &State<DbPool>) -> Option<content::RawJson<String>> {
    let conn = conn.get().expect("Failed to get DB connection");
    let data = get_data(&conn, key);
    data.map(|d| content::RawJson(d))
}

/// Post data to a key.
/// Example usage:
/// curl -X POST -H "Content-Type: application/json" -d '{"hi": "world"}' http://localhost:8000/name
#[post("/<key>", format = "json", data = "<data>")]
fn post(key: &str, data: Json<serde_json::Value>, conn: &State<DbPool>) -> content::RawJson<String>  {
    let conn = conn.get().expect("Failed to get DB connection");
    let data_str = data.to_string();
    put_data(&conn, key, &data_str).expect("Error putting data");
    content::RawJson(data_str)
}

#[launch]
fn rocket() -> _ {
    let manager = SqliteConnectionManager::file("kv.db");
    let pool = Pool::builder().max_size(15).build(manager).expect("Failed to create DB pool");
    let conn = pool.get().expect("Failed to get DB connection");
    create_schema(&conn);
    rocket::build().manage(pool).mount("/", routes![get, post])
}
