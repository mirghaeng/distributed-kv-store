#[macro_use] extern crate rocket;

mod models;
use models::{Value, StoreGetResponse};

use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use std::collections::HashMap;
use std::sync::RwLock;

type Store = RwLock<HashMap<String, String>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<key>")]
fn get_key(key: String) -> String {
    format!("get on /key-value-store/{}", key)
}

#[put("/<key>", format="application/json", data="<value>")]
fn put_key(key: String, value: Json<Value>) -> String {
    format!("put on /key-value-store/{} for value {}", key, value.value)
}

#[delete("/<key>")]
fn del_key(key: String) -> String {
    format!("delete on /key-value-store/{}", key)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(RwLock::new(HashMap::<String, String>::new()))
    .mount("/key-value-store", routes![index, get_key, put_key, del_key])
}
