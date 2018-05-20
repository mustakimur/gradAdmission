use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::error::Error;

pub mod models;
pub mod schema;

use self::models::{Application, FromImport, NewApplication};

pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn show_all() {
    use self::schema::ApplicationsTbl::dsl::*;

    let db_conn = connect_db();

    let results = ApplicationsTbl
        .load::<Application>(&db_conn)
        .expect("Error");

    println!("Display {} applicants", results.len());

    for app in results {
        println!("{:?}", app);
    }
}

fn get_index(header: &csv::StringRecord, title: &str) -> Option<usize> {
    for (i, item) in header.iter().enumerate() {
        if item == title {
            return Some(i);
        }
    }

    None
}

fn import_app(import: &FromImport)->Result<(), Box<Error>> {
    use self::schema::ApplicationsTbl;

    let mut new_app = NewApplication {
        emp_id: 0,
        applicant_id: 0,
        name: "None",
        dob: "Invalid",
        gender: "M",
        country: "US",
        program: "CS",
        degree: "PhD",
        interests: "",
        ug_university: "",
        ug_major: "",
        ug_degree: "",
        ug_gpa: 0.0f32,
        grad_university: "",
        grad_major: "",
        grad_degree: "",
        grad_gpa: 0.0f32,
        toefl_ielts: 0,
        gre_verb: 0,
        gre_quanti: 0,
        gre_combined: 0,
        decision: "Pending",
        advisor: "",
        assistantship: "",
        fte: 0.0f32,
        yearly_amount: 0,
    };

    new_app.emp_id = import.emp_id.parse::<i32>()?;

    new_app.applicant_id = import.applicant_id.parse::<i32>()?;

    new_app.name = import.name;
    new_app.dob = import.dob;
    new_app.gender = import.gender;
    new_app.country = import.country;

    if import.degree.starts_with("COPSCIEN") {
        new_app.program = "CS";
    } else if import.degree.starts_with("COPSINET") {
        new_app.program = "CNSA";
    } else if import.degree.starts_with("COPSISEC") {
        new_app.program = "SEC";
    } else if import.degree.starts_with("COPSICRM") {
        new_app.program = "CRIM";
    } else {
        new_app.program = "UNK";
    }


    if import.degree.ends_with("PD") {
        new_app.degree = "Ph.D";
    } else if import.degree.ends_with("MS") {
        new_app.degree = "M.S";
    } else if import.degree.ends_with("MT") {
        new_app.degree = "M.T";
    } else {
        new_app.degree = "UNK";
    }

    let db_conn = connect_db();
    diesel::insert_into(ApplicationsTbl::table)
        .values(&new_app)
        .execute(&db_conn)?;

    Ok(())
}

fn import_csv_error (path: &str) -> Result<(), Box<Error>> {
    // Build the CSV reader and iterate over each record.
    let mut file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut emp_id_idx = 0;
    let mut applicant_id_idx = 0;
    let mut name_idx = 0;
    let mut dob_idx = 0;
    let mut gender_idx = 0;
    let mut country_idx = 0;
    let mut degree_idx = 0;

    {
        let header = rdr.headers()?;
        emp_id_idx = get_index(header, "External_Id").expect("No External_Id");
        applicant_id_idx = get_index(header, "Ref").expect("No Ref");
        name_idx = get_index(header, "Name").expect("No Name");
        dob_idx = get_index(header, "Birthdate").expect("No Birthdate");
        gender_idx = get_index(header, "Sex").expect("No Sex");
        country_idx = get_index(header, "Primary Citizenship").expect("No Primary Citizenship");
        degree_idx = get_index(header, "Plan Desc").expect("No Plan Desc");
    }

    for result in rdr.records() {
        let record = result?;

        let import = FromImport {
            emp_id: &record[emp_id_idx],
            applicant_id: &record[applicant_id_idx],
            name: &record[name_idx],
            dob: &record[dob_idx],
            gender: &record[gender_idx],
            country: &record[country_idx],
            degree: &record[degree_idx],
        };

        let result = import_app(&import);

        if result.is_err() {
            //println!("{}", result.unwrap_err());
        }
    }

    Ok(())
}

pub fn import_csv () {
    let result = import_csv_error("data/2018_fall/Export.csv");

    if result.is_err() {
        println!("{}", result.unwrap_err());
    }
}