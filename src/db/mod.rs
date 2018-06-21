use std::error::Error;
use std::fs::File;
use std::ops::Deref;

use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use ammonia::clean;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::iter;
use std::{env, io};

pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn connect() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Database is located at {}", &database_url);

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Faile to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for Connection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<DbPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

//
// Access the applications table
//
pub use self::models::{Application, FromImport, NewApplication};
pub use self::schema::applications_tbl;

impl Application {
    pub fn read(connection: &SqliteConnection) -> Vec<Application> {
        applications_tbl::table
            .order(applications_tbl::applicant_id.asc())
            .load::<Application>(connection)
            .unwrap()
    }

    pub fn get(connection: &SqliteConnection, id: i32) -> Option<Application> {
        let results = applications_tbl::table
            .filter(applications_tbl::applicant_id.eq(id))
            .limit(1)
            .load::<Application>(connection);

        if !results.is_err() {
            let results = results.unwrap();

            for app in results {
                return Some(app);
            }
        }

        None
    }

    pub fn update(conn: &SqliteConnection, mut app: Application) -> bool {
        app.santize();

        diesel::update(applications_tbl::table.find(app.applicant_id))
            .set(&app)
            .execute(conn)
            .is_ok()
    }

    // Santize the inputs
    pub fn santize(&mut self) {
        self.name = clean(&self.name);
        self.dob = clean(&self.dob);
        self.gender = clean(&self.gender);
        self.country = clean(&self.country);
        self.program = clean(&self.program);
        self.degree = clean(&self.degree);
        self.interests = clean(&self.interests);
        self.ug_university = clean(&self.ug_university);
        self.ug_major = clean(&self.ug_major);
        self.ug_degree = clean(&self.ug_degree);
        self.grad_university = clean(&self.grad_university);
        self.grad_major = clean(&self.grad_major);
        self.grad_degree = clean(&self.grad_degree);
        self.gre = clean(&self.gre);
        self.decision = clean(&self.decision);
        self.advisor = clean(&self.advisor);
        self.assistantship = clean(&self.assistantship);
    }
}

//
// Access the comments table
//
pub use self::models::Comment;
pub use self::schema::comments_tbl;

impl Comment {
    pub fn read(connection: &SqliteConnection, id: i32) -> Vec<Comment> {
        comments_tbl::table
            .filter(comments_tbl::applicant_id.eq(id))
            .order(comments_tbl::comment_id.asc())
            .load::<Comment>(connection)
            .unwrap()
    }

    /*
    pub fn update(conn: &SqliteConnection, mut com: Comment) -> bool {
        com.santize();

        diesel::update(comments_tbl::table.find(com.applicant_id))
            .set(&com)
            .execute(conn)
            .is_ok()
    }*/

    pub fn insert(conn: &SqliteConnection, mut com: Comment) -> bool {
        com.santize();

        diesel::insert_into(comments_tbl::table)
            .values(&com)
            .execute(conn)
            .is_ok()
    }

    // Santize the inputs
    pub fn santize(&mut self) {
        self.commenter = clean(&self.commenter);
        self.opinion = clean(&self.opinion);

        // when is override by the server
    }
}

//
// Access the comments table
//
pub use self::models::User;
pub use self::schema::users_tbl;
use UserAuth;

impl User {
    fn gen_salt() -> String {
        let mut rng = rand::thread_rng();

        let salt: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(16)
            .collect();

        salt
    }

    pub fn hash_passwd(salt: &String, pass: &String) -> String {
        let ba = argon2rs::argon2d_simple(pass, salt);
        let strs: Vec<String> = ba.iter().map(|b| format!("{:02X}", b)).collect();
        strs.join("")
    }

    pub fn read(connection: &SqliteConnection) -> Vec<User> {
        users_tbl::table.load::<User>(connection).unwrap()
    }

    pub fn get(connection: &SqliteConnection, name: &str) -> Option<User> {
        let results = users_tbl::table
            .filter(users_tbl::user_name.eq(name))
            .limit(1)
            .load::<User>(connection);

        if !results.is_err() {
            let results = results.unwrap();

            for user in results {
                return Some(user);
            }
        }

        None
    }

    // This function is used to retrieve the user from the cookie. We do not have a connection.
    pub fn get_auth<'a, 'r>(request: &'a Request<'r>, name: &str) -> Option<UserAuth> {
        let pool_orig = request.guard::<State<DbPool>>();

        if pool_orig.is_success() {
            if let Ok(conn) = pool_orig.unwrap().get() {
                println!("user_name in cookie:{}", name);

                return User::get(&conn, name).map(|user| UserAuth {
                    user_name: user.user_name,
                    role: user.role,
                });
            }
        }

        None
    }

    /*pub fn update(conn: &SqliteConnection, mut user: User) -> bool {
        user.santize();

        diesel::update(users_tbl::table.find(&user.user_name))
            .set(&user)
            .execute(conn)
            .is_ok()
    }*/

    pub fn delete(conn: &SqliteConnection, name: &str) -> bool {
        diesel::delete(users_tbl::table.find(name))
            .execute(conn)
            .is_ok()
    }

    pub fn insert(conn: &SqliteConnection, mut user: User) -> bool {
        user.santize();
        user.salt = User::gen_salt();

        user.password = User::hash_passwd(&user.salt, &user.password);
        diesel::insert_into(users_tbl::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn santize(&mut self) {
        self.user_name = clean(&self.user_name);
        self.role = clean(&self.role);
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
/*
fn import_app(db_conn: &SqliteConnection, import: &FromImport) -> Result<String, Box<Error>> {
    use self::schema::applications_tbl;

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
        ug_gpa: 0.0f64,
        grad_university: "",
        grad_major: "",
        grad_degree: "",
        grad_gpa: 0.0f64,
        toefl_ielts: 0,
        gre: "0/0/0",
        decision: "Pending",
        advisor: "",
        assistantship: "None",
        fte: 0.0f64,
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

    diesel::insert_into(applications_tbl::table)
        .values(&new_app)
        .execute(db_conn)?;

    Ok("Success".to_string())
}
*/

pub fn import_csv(db_conn: &SqliteConnection, path: &str) -> io::Result<String> {
    use self::schema::applications_tbl;

    // Build the CSV reader and iterate over each record.
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let fields = vec![
        "Ref",
        "Name",
        "Birthdate",
        "Academic Department",
        "Plan",
        "External_Id",
        "Primary Citizenship",
        "Sex",
        "Email",
        "School 1 Institution",
        "School 1 Major",
        "School 1 Degree",
        "School 2 Institution",
        "School 2 Major",
        "School 2 Degree",
        "TOEFL Total",
        "TOEFL Listening (0-30)",
        "TOEFL Reading (0-30)",
        "TOEFL Structure/Written Expression",
        "TOEFL Speaking (0-30)",
        "GRE Verbal (130-170)",
        "GRE Quantitative (130-170)",
        "GRE Analytical Writing (0-6)",
    ];

    let f2idx = HashMap::new();
    let header = rdr.headers()?;

    for f in fields {
        f2idx.insert(f, get_index(header, f));
    }

    let read_field = |record, idx| match idx {
        Some(i) => clean(&record[i]),
        None => "".to_string(),
    };

    let toefl = vec![
        "TOEFL Total",
        "TOEFL Listening (0-30)",
        "TOEFL Reading (0-30)",
        "TOEFL Structure/Written Expression",
        "TOEFL Speaking (0-30)",
    ];

    let gre = vec![
        "GRE Verbal (130-170)",
        "GRE Quantitative (130-170)",
        "GRE Analytical Writing (0-6)",
    ];

    for r in rdr.records() {
        let record = r?;

        let mut new_app = Application {
            emp_id: read_field(&record, f2idx.get("External_Id")).parse::<i32>()?,
            applicant_id: read_field(&record, f2idx.get("Ref")).parse::<i32>()?,
            name: read_field(&record, f2idx.get("Name")),
            dob: read_field(&record, f2idx.get("Birthdate")),
            gender: read_field(&record, f2idx.get("Sex")),
            country: read_field(&record, f2idx.get("Primary Citizenship")),
            program: read_field(&record, f2idx.get("Plan")),
            degree: "".to_string(),
            interests: "".to_string(),
            ug_university: read_field(&record, f2idx.get("School 2 Institution")),
            ug_major: read_field(&record, f2idx.get("School 2 Major")),
            ug_degree: read_field(&record, f2idx.get("School 2 Degree")),
            ug_gpa: 0.0f64,
            grad_university: read_field(&record, f2idx.get("School 1 Institution")),
            grad_major: read_field(&record, f2idx.get("School 1 Major")),
            grad_degree: read_field(&record, f2idx.get("School 1 Degree")),
            grad_gpa: 0.0f64,
            toefl_ielts: toefl
                .iter()
                .map(|f| read_field(&record, f2idx.get(f)))
                .collect()
                .join("/"),
            gre: gre
                .iter()
                .map(|f| read_field(&record, f2idx.get(f)))
                .collect()
                .join("/"),
            decision: "Pending".to_string(),
            advisor: "".to_string(),
            assistantship: "None".to_string(),
            fte: 0.0f64,
            yearly_amount: 0,
        };

        // parse the degree applied to
        let degree = &new_app.degree;
        let program = "";

        if degree.starts_with("COPSCIEN") {
            program = "CS";
        } else if degree.starts_with("COPSINET") {
            program = "CNSA";
        } else if degree.starts_with("COPSISEC") {
            program = "SEC";
        } else if degree.starts_with("COPSICRM") {
            program = "CRIM";
        } else {
            program = "UNK";
        }

        new_app.program = program.to_string();

        if degree.ends_with("PD") {
            degree = "Ph.D";
        } else if degree.ends_with("MS") {
            degree = "M.S";
        } else if import.degree.ends_with("MT") {
            degree = "M.T";
        } else {
            degree = "UNK";
        }

        new_app.degree = degree.to_string();

        // update the degree
        if new_app.grad_degree.starts_with("B") {
            new_app.ug_university = new_app.grad_university;
            new_app.ug_major = new_app.grad_major;
            new_app.ug_degree = new_app.grad_degree;
            new_app.ug_gpa = new_app.grad_gpa;

            new_app.grad_university = "".to_string();
            new_app.grad_major = "".to_string();
            new_app.grad_degree = "".to_string();
            new_app.grad_gpa = "".to_string();
        }

        println!("{:?}", new_app);

        let result = diesel::insert_into(applications_tbl::table)
            .values(&new_app)
            .execute(db_conn);

        if result.is_err() {
            println!("{}", result.unwrap_err());
        }
    }

    Ok("Success".to_string())
}
