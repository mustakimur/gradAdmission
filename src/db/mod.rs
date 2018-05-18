use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{Application, NewApplication};

pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn show_all() {
    use self::schema::ApplicationsTbl::dsl::*;

    let db_conn = connect_db();

    let results = ApplicationsTbl
        .limit(5)
        .load::<Application>(&db_conn)
        .expect("Error");

    println!("Display {} applicants", results.len());

    for app in results {
        println!("{:?}", app);
    }
}

pub fn create_app() -> usize {
    use self::schema::ApplicationsTbl;

    let new_app = NewApplication {
        EmpID: 90202,
        ApplicantID: 90090,
        Name: "Wang, Zhi".to_string(),
        DOB: "05/12/1990".to_string(),
        Gender: "M".to_string(),
        Country: "CHN".to_string(),
        Program: "CS".to_string(),
        Degree: "PhD".to_string(),
        Interests: "Security".to_string(),
        UG_University: "Xi'an Jiaotong University".to_string(),
        UG_Major: "CS".to_string(),
        UG_Degree: "BS".to_string(),
        UG_GPA: 3.8f32,
        Grad_University: "Xi'an Jiaotong University".to_string(),
        Grad_Major: "CS".to_string(),
        Grad_Degree: "MS".to_string(),
        Grad_GPA:3.8f32,
        TOEFL_IELTS: 140,
        GRE_Verb: 500,
        GRE_Quanti: 500,
        GRE_Combined: 1400,
        Decision: "Pending".to_string(),
        Advisor: "Xuxian Jiang".to_string(),
        Assistantship: "RA".to_string(),
        FTE: 0.5,
        YearlyAmount: 22000,
    };

    let db_conn = connect_db();
    diesel::insert_into(ApplicationsTbl::table)
        .values(&new_app)
        .execute(&db_conn)
        .expect("error")
}
