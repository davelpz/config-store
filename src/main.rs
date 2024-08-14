mod db;
use db::{connect, create_schema, get_data, put_data};
use rocket::response::content;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Get data from a key.
#[get("/<key>")]
fn get(key: &str) -> Option<content::RawJson<String>> {
    let conn = connect();
    let data = get_data(&conn, key);
    data.map(|d| content::RawJson(d))
}

/// Post data to a key.
/// Example usage:
/// curl -X POST -H "Content-Type: application/json" -d '{"hi": "world"}' http://localhost:8000/name
#[post("/<key>", format = "json", data = "<data>")]
fn post(key: &str, data: Json<serde_json::Value>) -> content::RawJson<String>  {
    let conn = connect();
    let data_str = data.to_string();
    put_data(&conn, key, &data_str).expect("Error putting data");
    content::RawJson(data_str)
}

#[launch]
fn rocket() -> _ {
    create_schema(&connect());
    rocket::build().mount("/", routes![index, get, post])
}
