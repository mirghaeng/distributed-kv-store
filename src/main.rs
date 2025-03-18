#[macro_use] extern crate rocket;

mod models;
use models::{Value, StoreGetResponse, StorePutResponse, StoreDeleteResponse};

use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use std::collections::HashMap;
use std::sync::Mutex;

// A key-value store with causal consistency typically involves more writes than reads
// Mutex for frequent writes, RwLock for read-heavy workloads
type StoreType = Mutex<HashMap<String, String>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<key>")]
fn get_key(store: &State<StoreType>, key: String) -> status::Custom<Json<StoreGetResponse>> {
    let store = store.lock().unwrap();
    
    // Case: Key does not exist, Return: 404 Not found
    if !store.contains_key(&key) {
        return status::Custom(Status::NotFound, Json(StoreGetResponse {
            does_exist: false,
            error: Some(String::from("Key does not exist")),
            message: Some(String::from("Error in GET")),
            value: None
        }))
    }

    // Case: Key exists, Return: 200 OK
    status::Custom(Status::Ok, Json(StoreGetResponse {
        does_exist: true,
        error: None,
        message: Some(String::from("Retrieved successfully")),
        value: store.get(&key).map(|s| s.into()),
    }))
}

#[put("/<key>", format="application/json", data="<val_op>")]
fn put_key(store: &State<StoreType>, key: String, val_op: Option<Json<Value>>) -> status::Custom<Json<StorePutResponse>> {
    let mut store = store.lock().unwrap();

    match val_op {
        
        // Case: Value not specified, Return: 400 Bad Request
        None => status::Custom(Status::BadRequest, Json(StorePutResponse {
            message: String::from("Error in PUT"),
            replaced: None,
            error: Some(String::from("Value is missing")),
        })),
        Some(value) => {

            // Case: Key length is greater than 50, Return 400 Bad Request
            if key.len() > 50 {
                status::Custom(Status::BadRequest, Json(StorePutResponse {
                    message: String::from("Error in PUT"),
                    replaced: None,
                    error: Some(String::from("Key is too long")),
                }))

            // Case: Key exists, value specified, Return 200 OK
            } else if store.contains_key(&key) {
                store.insert(key, value.value.clone());
                status::Custom(Status::Ok, Json(StorePutResponse {
                    message: String::from("Updated successfully"),
                    replaced: Some(true),
                    error: None,
                }))

            // Case: Key does not exist, Return: 201 Created
            } else {
                store.insert(key, value.value.clone());
                status::Custom(Status::Created, Json(StorePutResponse {
                    message: String::from("Added successfully"),
                    replaced: Some(false),
                    error: None,
                }))
            }
        }
    }
}

#[delete("/<key>")]
fn del_key(store: &State<StoreType>, key: String) -> status::Custom<Json<StoreDeleteResponse>> {
    let mut store = store.lock().unwrap();

    // Case: Key does not exist, Return: 404 Not Found
    if !store.contains_key(&key) {
        return status::Custom(Status::NotFound, Json(StoreDeleteResponse {
            does_exist: false,
            error: Some(String::from("Key does not exist")),
            message: Some(String::from("Error in DELETE")),
            value: None,
        }))
    }

    store.remove(&key);

    // Case: Key exists, Return: 200 OK
    status::Custom(Status::Ok, Json(StoreDeleteResponse {
        does_exist: true,
        error: None,
        message: Some(String::from("Deleted successfully")),
        value: None,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(Mutex::new(HashMap::<String, String>::new()))
    .mount("/key-value-store", routes![index, get_key, put_key, del_key])
}
