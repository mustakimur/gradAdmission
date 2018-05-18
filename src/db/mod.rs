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

    let new_app = NewApp {
        id: 90202,
        first_name: "Zhi".to_string(),
        last_name: "Wang",
        program: "MS",
        nationality: "CHN",
        sex: "F",
        under_college: "Xi'an Jiaotong University",
        under_gpa: 3.4f32,
        ms_college: None,
        ms_gpa: None,
        interests: "Security",
        decision: "pending",
    };

    let new_app = NewApplication {
        EmpId: 90202,
        ApplicationId: 90090,
        Name: "Wang, Zhi".to_string(),
        DOC: "05/12/1990".to_string(),
        Gender: "M".to_string(),
        Country: "CHN",
        Program: "CS",
        Degree: "CS",
        Interests: "CS",
        UG_University: "CS",
        UG_Major: "CS",
        UG_GPA: "CS",
        Grad_University: "CS",
        Grad_Major: "CS",
        Grad_Degree: "CS",
        TOEFL_IELTS: "CS",
        GRE_Verb: "CS",
        GRE_Quanti: "CS",
        GRE_Combined: "CS",
        Decision: "CS",
        Advisor: "CS",
        FTE: "CS",
        YearlyAmount: "CS",
    };

    let db_conn = connect_db();
    diesel::insert_into(basicinfo::table)
        .values(&new_app)
        .execute(&db_conn)
        .expect("error")
}
