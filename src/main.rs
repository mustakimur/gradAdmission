#![feature(plugin,custom_derive)]
#![plugin(rocket_codegen)]
#![feature(extern_prelude)]
#![recursion_limit="128"]

extern crate rocket;
extern crate csv;
extern crate handlebars;
extern crate chrono;

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_diesel;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket_contrib::{Json, Value, Template};
use chrono::Local;

pub mod db;
use db::{Application, Comment};

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
fn detail(id: i32, connection: db::Connection) -> Template { 
    let one = Application::get(&connection, id).unwrap();
    Template::render("review", &one)
}


#[get("/<_id>/<file..>")]
fn read_file(_id: i32, file: PathBuf) -> Option<NamedFile> {    
    let mut path = Path::new("data/2018_fall/").join(&file);
    path.set_extension("pdf");
    println!("{}", path.to_str().unwrap());
    NamedFile::open(path).ok()
}

#[get("/<id>")]
fn all_comment(connection: db::Connection, id: i32)->Json<Value> {
   Json(json!(Comment::read(&connection, id)))
}

#[post("/<_id>", data = "<cmt>")]
fn add_comment(connection: db::Connection, _id: i32, cmt: Json<Comment>)->Json<Value> {
    let date = Local::now ();
    let now = date.format("%m/%d/%Y %H:%M:").to_string();
    let c = Comment{comment_id: None, when: now, ..cmt.into_inner()};
    Comment::add_one(&connection, c);
   Json(json!({"status": "success"}))
}


fn main() {
    db::import_csv();
    rocket::ignite()
        .mount("/", routes![index, login, mainpg, resources,images,detail])
        .mount("/apps", routes![read_all, read_one, update_one])
        .mount("/files", routes![read_file])
        .mount("/comments", routes![all_comment, add_comment])
        .manage(db::connect())
        .attach(Template::fairing())
        .launch();
}
