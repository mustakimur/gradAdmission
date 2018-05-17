use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Basicinfo, NewApp};

pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn show_all() {
    use self::schema::basicinfo::dsl::*;

    let db_conn = connect_db();
    
    let results = basicinfo.limit(5).load::<Basicinfo>(&db_conn).expect("Error");

    println!("Display {} applicants",results.len());

    for bi in results {
        println!("{:?}", bi);
    }

}

pub fn create_app() ->usize {
    use self::schema::basicinfo;

    let new_app = NewApp{
    id: 90202,
    first_name: "Zhi".to_string(),
    last_name: "Wang",
    program:  "MS",
    nationality:  "CHN",
    sex:  "F",
    under_college:  "Xi'an Jiaotong University",
    under_gpa: 3.4f32,
    ms_college: None,
    ms_gpa: None,
    interests:  "Security",
    decision:  "pending",
    };

    let db_conn = connect_db();
    diesel::insert_into(basicinfo::table).values(&new_app).execute(&db_conn).expect("error")

}