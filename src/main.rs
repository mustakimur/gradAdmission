#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(extern_prelude)]
#![recursion_limit = "128"]

extern crate chrono;
extern crate csv;
extern crate handlebars;
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
extern crate argon2rs;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ammonia;

use chrono::Local;
use rocket::http::{Status, Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, Form, FromRequest, Request};
use rocket::response::{Failure, Flash, NamedFile, Redirect};
use rocket::Data;
use rocket_contrib::{Json, Template, Value};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub mod db;
use db::{Application, Comment, User};

//
// Routers to handle login and logout
//
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuth {
    user_name: String,
    role: String,
}

impl UserAuth {
    fn is_sys(&self) -> bool {
        return self.role == "Sys";
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAuth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserAuth, ()> {
        request
            .cookies()
            .get_private("user_name")
            .and_then(|cookie| User::get_auth(request, cookie.value()))
            .or_forward(())
    }
}

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref DATA_DIR: String = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let path = Path::new(&database_url)
            .parent()
            .expect("Invalid directory for DATABASE_URL");

        if !path.is_dir() {
            panic!("Invalid directory for data");
        }

        path.to_str().expect("Invalid directory for data").to_string()
    };
}

#[get("/login")]
fn login_auth(_user: UserAuth) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

#[derive(FromForm, Debug)]
struct Login {
    user_name: String,
    password: String,
    remember: Option<String>,
}

#[post("/login", data = "<lg>")]
fn login(mut cookies: Cookies, lg: Form<Login>, connection: db::Connection) -> Flash<Redirect> {
    let name = (&lg.get().user_name).to_string();
    let user_opt = User::get(&connection, &name);

    /*  println!(
        "User password after hash: {}",
        User::hash_passwd(&lg.get().password)
    ); */

    if let Some(user) = user_opt {
        if user.password == User::hash_passwd(&lg.get().password) {
            cookies.add_private(Cookie::new("user_name", name));
            return Flash::success(Redirect::to("/"), "Successfully logged in.");
        }
    }

    Flash::error(Redirect::to("/login"), "Invalid username/password.")
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_name"));
    Redirect::to("/login")
}

//
// Routers to handle url based on /
//
#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/login")
}

#[get("/images/<_file..>", rank = 2)]
fn images(_file: PathBuf) -> Redirect {
    Redirect::to("/login")
}

#[get("/")]
fn index_auth(user: UserAuth) -> Template {
    Template::render("applicants", &user)
}

#[get("/resources/<file..>")]
fn resources(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("html/resources/").join(&file);
    NamedFile::open(path).ok()
}

#[get("/images/<file..>")]
fn images_auth(file: PathBuf, _user: UserAuth) -> Option<NamedFile> {
    let path = Path::new("html/images/").join(&file);
    NamedFile::open(path).ok()
}

//
// Routers to handle urls based on /application
//
#[get("/")]
fn read_apps_auth(connection: db::Connection, _user: UserAuth) -> Json<Value> {
    Json(json!(Application::read(&connection)))
}

#[get("/<id>")]
fn read_app_auth(id: i32, connection: db::Connection, _user: UserAuth) -> Json<Value> {
    let one = Application::get(&connection, id);
    if let Some(app) = one {
        Json(json!(app))
    } else {
        Json(json!({"status": "Error", "message": "not found"}))
    }
}

#[post("/", data = "<app>")]
fn update_app_auth(
    app: Json<Application>,
    connection: db::Connection,
    _user: UserAuth,
) -> Json<Value> {
    let new_app = Application { ..app.into_inner() };
    if Application::update(&connection, new_app) {
        Json(json!({"status": "Success"}))
    } else {
        Json(json!({"status": "Error", "message" : "failed to update not found"}))
    }
}

#[get("/", rank = 2)]
fn read_apps(_connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

#[get("/<_id>", rank = 2)]
fn read_app(_id: i32, _connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

#[post("/", data = "<_app>", rank = 2)]
fn update_app(_app: Json<Application>, _connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

//
// Routers to handle urls based on /review
//
#[get("/<id>")]
fn review_app_auth(id: i32, connection: db::Connection, _user: UserAuth) -> Template {
    let one = Application::get(&connection, id).unwrap();
    Template::render("review", &one)
}

#[get("/<_id>", rank = 2)]
fn review_app(_id: i32, _connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

//
// Routers to handle urls based on /file
//
#[get("/<id>/<file..>")]
fn read_file_auth(id: i32, file: PathBuf, _user: UserAuth) -> Option<NamedFile> {
    //println!("Data_dir {:?}", *DATA_DIR);

    let mut path = Path::new(&*DATA_DIR).join(id.to_string());

    path.push(file);
    path.set_extension("pdf");

    //println!("{}", path.to_str().unwrap());
    NamedFile::open(path).ok()
}

#[get("/<_id>/<_file..>", rank = 2)]
fn read_file(_id: i32, _file: PathBuf) -> Redirect {
    Redirect::to("/login")
}

#[get("/<id>")]
fn read_index_auth(id: i32, _user: UserAuth) -> String {
    let path = Path::new(&*DATA_DIR).join(id.to_string());
    let mut index = "".to_string();

    if !path.is_dir() {
        return index;
    }

    let dir_iter = fs::read_dir(path);

    if dir_iter.is_err() {
        return index;
    }

    let dir_iter = dir_iter.unwrap();

    for entry in dir_iter {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();

        let path = entry.path();
        if path.is_file() {
            if let Some(Some(fname)) = path.file_name().map(|osstr| osstr.to_str()) {
                if index.len() > 0 {
                    index.push(',');
                }
                index.push_str(&fname);
            }
        }
    }

    println!("read_index: {}", index);
    index
}

#[get("/<_id>", rank = 2)]
fn read_index(_id: i32) -> Redirect {
    Redirect::to("/login")
}

#[post("/<id>/<file..>", data = "<data>")]
fn write_file_auth(data: Data, id: i32, file: PathBuf, _user: UserAuth) -> io::Result<String> {
    let path = Path::new(&*DATA_DIR).join(id.to_string());

    fs::create_dir_all(&path)?;

    let mut path = path.join(&file);
    path.set_extension("pdf");
    //println!("Write file: {}", path.to_str().unwrap());

    data.stream_to_file(path)
        .map(|n| format!("Saved {} bytes to {}", n, file.display()))
}

#[post("/<_id>/<_file..>", rank = 2)]
fn write_file(_id: i32, _file: PathBuf) -> Redirect {
    Redirect::to("/login")
}

#[post("/import", data = "<paste>")]
fn import_auth(paste: Data, connection: db::Connection, user: UserAuth) -> io::Result<String> {
    let filename = format!("{}/import.csv", &*DATA_DIR);
    println!(
        "in import, with user {}, save file to {}",
        user.user_name, &filename
    );

    // Write the paste out to the file and return the URL.
    paste.stream_to_file(Path::new(&filename))?;
    db::import_csv(&connection, &filename)
}

#[post("/import", data = "<_paste>", rank = 2)]
fn import(_paste: Data) -> Redirect {
    println!("in import, without user");
    Redirect::to("/login")
}
//
// Routers to handle urls based on /comment
//
#[get("/<id>")]
fn read_comments_auth(id: i32, connection: db::Connection, _user: UserAuth) -> Json<Value> {
    Json(json!(Comment::read(&connection, id)))
}

#[post("/<_id>", data = "<cmt>")]
fn add_comment_auth(
    connection: db::Connection,
    _id: i32,
    cmt: Json<Comment>,
    user: UserAuth,
) -> Json<Value> {
    let date = Local::now();
    let now = date.format("%m/%d/%Y %H:%M:").to_string();
    let mut c = Comment {
        comment_id: None,
        when: now,
        ..cmt.into_inner()
    };

    if user.user_name != c.commenter {
        c.commenter = format!("{} B/O {}", user.user_name, c.commenter);
    }

    if Comment::insert(&connection, c) {
        Json(json!({"status": "Success"}))
    } else {
        Json(json!({"status": "Error", "message" : "Failed to insert the comment"}))
    }
}

#[get("/<_id>", rank = 2)]
fn read_comments(_connection: db::Connection, _id: i32) -> Redirect {
    Redirect::to("/login")
}

#[post("/<_id>", data = "<_cmt>", rank = 2)]
fn add_comment(_connection: db::Connection, _id: i32, _cmt: Json<Comment>) -> Redirect {
    Redirect::to("/login")
}

//
// Routers to handle urls based on /users
//
#[get("/")]
fn manage_user_auth(_connection: db::Connection, user: UserAuth) -> Result<Template, Failure> {
    if !user.is_sys() {
        Err(Failure(Status::Forbidden))
    } else {
        Ok(Template::render("user", &user))
    }
}

#[get("/", rank = 2)]
fn manage_user(_connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

#[get("/")]
fn read_users_auth(connection: db::Connection, user: UserAuth) -> Json<Value> {
    if !user.is_sys() {
        return Json(
            json!({"status": "Error", "message" : "only sys admin can change user settings"}),
        );
    }

    let mut users = User::read(&connection);

    for u in &mut users {
        u.password = "Password hash hidden".to_string();
    }

    Json(json!(users))
}

#[get("/", rank = 2)]
fn read_users(_connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

#[post("/", data = "<new_user>")]
fn add_user_auth(new_user: Json<User>, connection: db::Connection, user: UserAuth) -> Json<Value> {
    let new_user = User {
        ..new_user.into_inner()
    };

    if user.is_sys() && User::insert(&connection, new_user) {
        Json(json!({"status": "Success"}))
    } else {
        Json(json!({"status": "Error", "message" : "failed to add user"}))
    }
}

#[post("/", data = "<_new_user>", rank = 2)]
fn add_user(_new_user: Json<User>, _connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

#[delete("/<user_name>")]
fn del_user_auth(user_name: String, connection: db::Connection, user: UserAuth) -> Json<Value> {
    if user.is_sys() && User::delete(&connection, &user_name) {
        Json(json!({"status": "Success"}))
    } else {
        Json(json!({"status": "Error", "message" : "failed to delete user"}))
    }
}

#[delete("/<_user_name>", rank = 2)]
fn del_user(_user_name: String, _connection: db::Connection) -> Redirect {
    Redirect::to("/login")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                index_auth,
                login,
                login_auth,
                login_page,
                logout,
                resources,
                images,
                images_auth,
            ],
        )
        .mount(
            "/application",
            routes![
                read_apps,
                read_apps_auth,
                read_app,
                read_app_auth,
                update_app,
                update_app_auth
            ],
        )
        .mount("/review", routes![review_app, review_app_auth])
        .mount(
            "/file",
            routes![
                read_file,
                read_file_auth,
                import_auth,
                import,
                write_file,
                write_file_auth,
                read_index,
                read_index_auth
            ],
        )
        .mount(
            "/user",
            routes![
                read_users,
                read_users_auth,
                add_user,
                add_user_auth,
                del_user,
                del_user_auth
            ],
        )
        .mount("/account", routes![manage_user, manage_user_auth])
        .mount(
            "/comment",
            routes![
                read_comments,
                read_comments_auth,
                add_comment,
                add_comment_auth
            ],
        )
        .manage(db::connect())
        .attach(Template::fairing())
        .launch();
}
