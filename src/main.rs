mod db;
use db::{create_schema, get_data, put_data, delete_data, get_all_keys, count_keys};
use rocket::response::content;
use rocket::State;
use rocket::serde::json::Json;
use r2d2::{Pool};
use r2d2_sqlite::SqliteConnectionManager;

#[macro_use] extern crate rocket;

type DbPool = Pool<SqliteConnectionManager>;

/// Get data from a key.
/// Example usage:
/// curl http://localhost:8000/name
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

/// Put data to a key. same as post.
/// Example usage:
/// curl -X PUT -H "Content-Type: application/json" -d '{"hi": "world"}' http://localhost:8000/name
#[put("/<key>", format = "json", data = "<data>")]
fn put(key: &str, data: Json<serde_json::Value>, conn: &State<DbPool>) -> content::RawJson<String> {
    post(key, data, conn)
}

/// Delete key and data from the database.
/// Example usage:
/// curl -X DELETE http://localhost:8000/name
#[delete("/<key>")]
fn delete(key: &str, conn: &State<DbPool>) -> Option<content::RawJson<String>> {
    let conn = conn.get().expect("Failed to get DB connection");
    match delete_data(&conn, key) {
        Ok(count) => {
            if count == 0 {
                None
            } else {
                Some(content::RawJson("{\"status\": \"deleted\"}".to_string()))
            }
        },
        Err(e) => Some(content::RawJson(format!("{{\"status\": \"error\", \"msg\": \"{}\"}}", e)))
    }
}

/// Check if a key exists.
/// Example usage:
/// curl -I http://localhost:8000/name
#[head("/<key>")]
fn check_key(key: &str, conn: &State<DbPool>) -> rocket::http::Status {
    let conn = conn.get().expect("Failed to get DB connection");
    if get_data(&conn, key).is_some() {
        rocket::http::Status::Ok
    } else {
        rocket::http::Status::NotFound
    }
}

/// List all keys in the database.
/// Example usage:
/// curl http://localhost:8000/keys
#[get("/keys")]
fn list_keys(conn: &State<DbPool>) -> Json<Vec<String>> {
    let conn = conn.get().expect("Failed to get DB connection");
    let keys = get_all_keys(&conn);
    Json(keys)
}

/// Count the number of keys in the database.
/// Example usage:
/// curl http://localhost:8000/count
#[get("/count")]
fn key_count(conn: &State<DbPool>) -> Json<usize> {
    let conn = conn.get().expect("Failed to get DB connection");
    let count = count_keys(&conn);
    Json(count)
}

#[launch]
fn rocket() -> _ {
    let manager = SqliteConnectionManager::file("kv.db");
    let pool = Pool::builder().max_size(15).build(manager).expect("Failed to create DB pool");
    let conn = pool.get().expect("Failed to get DB connection");
    create_schema(&conn);
    rocket::build().manage(pool).mount("/", routes![get, post, put, delete, list_keys, key_count, check_key])
}
