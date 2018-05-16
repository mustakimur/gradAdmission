#![feature(plugin,custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;

// #[cfg(test)] mod tests;

// use rocket_contrib::{Json, Value};
// use rocket::State;
// use std::collections::HashMap;
// use std::sync::Mutex;

use std::io;
use rocket::response::NamedFile;
use rocket::response::Redirect;

// The type to represent the ID of a message.
//type ID = usize;

// We're going to store all of the messages here. No need for a DB.
//type MessageMap = Mutex<HashMap<ID, String>>;

/* #[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "application/json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<MessageMap>) -> Json<Value> {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        Json(json!({
            "status": "error",
            "reason": "ID exists. Try put."
        }))
    } else {
        hashmap.insert(id, message.0.contents);
        Json(json!({ "status": "ok" }))
    }
}

#[put("/<id>", format = "application/json", data = "<message>")]
fn update(id: ID, message: Json<Message>, map: State<MessageMap>) -> Option<Json<Value>> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(Json(json!({ "status": "ok" })))
    } else {
        None
    }
}

#[get("/<id>", format = "application/json")]
fn get(id: ID, map: State<MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
            id: Some(id),
            contents: contents.clone()
        })
    })
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
} */

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("html/login.html")
}

#[get("/main")]
fn mainpg() -> io::Result<NamedFile> {
    NamedFile::open("html/applicants.html")
}

#[derive(FromForm,Debug)]
struct User{
    user: String,
    password: String,
    remember: Option<String>
}

#[get("/login?<user>")]
fn login(user: User) -> Redirect {
    println!("{:?}", user);
    Redirect::to("/main")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, login, mainpg])
       // .catch(errors![not_found])
       // .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
