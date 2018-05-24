#![feature(plugin,custom_derive)]
#![plugin(rocket_codegen)]
#![feature(extern_prelude)]
#![recursion_limit="128"]

extern crate rocket;
extern crate dotenv;
extern crate csv;

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_diesel;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket_contrib::{Json, Value};

pub mod db;
use db::{Application};

// Serving basic files
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("html/login.html")
}

#[get("/main")]
fn mainpg() -> io::Result<NamedFile> {
    NamedFile::open("html/applicants.html")
}

#[get("/resources/<file..>")]
fn resources(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("html/resources/").join(&file);
    NamedFile::open(path).ok()
}

#[get("/images/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {    
    let path = Path::new("html/images/").join(&file);
    NamedFile::open(path).ok()
}

// handle login
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

// CRUD applications
#[get("/")]
fn read_all(connection: db::Connection)->Json<Value> {
   Json(json!(Application::read(&connection)))
}

#[get("/<id>")]
fn read_one(connection: db::Connection, id: i32)->Json<Value> {
    let one = Application::get(&connection, id);
    if let Some(app) = one {
        Json(json!(app))
    } else {
        Json(json!({"status": "error - not found"}))
    }
}


#[post("/", data = "<app>")]
fn update_one(app: Json<Application>, connection: db::Connection) -> Json<Value> {
    let p = Application{..app.into_inner()};
    Application::update(&connection, p);
    Json(json!({"status": "error - not found"}))
}

#[get("/review/<id>")]
fn detail(id: i32) -> Option<NamedFile> {    
    NamedFile::open("html/review.html").ok()
}

fn main() {
    //db::import_csv();
    rocket::ignite()
        .mount("/", routes![index, login, mainpg, resources,images,detail])
        .mount("/apps", routes![read_all, read_one, update_one])
        .manage(db::connect())
        .launch();
}
