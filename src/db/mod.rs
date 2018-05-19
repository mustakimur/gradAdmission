use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind};

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
        emp_id: 90202,
        applicant_id: 90090,
        name: "Wang, Zhi".to_string(),
        dob: "05/12/1990".to_string(),
        gender: "M".to_string(),
        country: "CHN".to_string(),
        program: "CS".to_string(),
        degree: "PhD".to_string(),
        interests: "Security".to_string(),
        ug_university: "Xi'an Jiaotong University".to_string(),
        ug_major: "CS".to_string(),
        ug_degree: "BS".to_string(),
        ug_gpa: 3.8f32,
        grad_university: "Xi'an Jiaotong University".to_string(),
        grad_major: "CS".to_string(),
        grad_degree: "MS".to_string(),
        grad_gpa: 3.8f32,
        toefl_ielts: 140,
        gre_verb: 500,
        gre_quanti: 500,
        gre_combined: 1400,
        decision: "Pending".to_string(),
        advisor: "Xuxian Jiang".to_string(),
        assistantship: "RA".to_string(),
        fte: 0.5,
        yearly_amount: 22000,
    };

    let db_conn = connect_db();
    diesel::insert_into(ApplicationsTbl::table)
        .values(&new_app)
        .execute(&db_conn)
        .expect("error")
}

fn get_index(header: &csv::StringRecord, title: &str) -> Option<usize> {
    for (i, item) in header.iter().enumerate() {
        if item == title {
            return Some(i);
        }
    }

    None
}

pub fn import_csv() -> Result<(), Error> {
    // Build the CSV reader and iterate over each record.
    let mut file = File::open("data/2018_fall/Export.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut emp_id_idx = 0;
    let mut applicant_id_idx = 0;
    let mut name_idx = 0;
    let mut dob_idx = 0;
    let mut gender_idx = 0;
    let mut country_idx = 0;
    let mut degree_idx = 0;

    if !rdr.has_headers() {
        return Err(io::Error::new(ErrorKind::Other, "import must have header!"));
    }

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

        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            &record[emp_id_idx],
            &record[applicant_id_idx],
            &record[name_idx],
            &record[dob_idx],
            &record[gender_idx],
            &record[country_idx],
            &record[degree_idx]
        );
    }

    Ok(())
}
