use super::schema::applications_tbl;
use super::schema::comments_tbl;
use super::schema::users_tbl;

#[derive(Queryable, AsChangeset, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "applications_tbl"]
pub struct Application {
    pub emp_id: i32,
    pub applicant_id: i32,
    pub name: String,
    pub dob: String,
    pub gender: String,
    pub country: String,
    pub program: String,
    pub degree: String,
    pub term: String,
    pub interests: String,
    pub ug_university: String,
    pub ug_major: String,
    pub ug_degree: String,
    pub ug_gpa: f64,
    pub grad_university: String,
    pub grad_major: String,
    pub grad_degree: String,
    pub grad_gpa: f64,
    pub toefl_ielts: String,
    pub gre: String,
    pub decision: String,
    pub advisor: String,
    pub assistantship: String,
    pub fte: f64,
    pub yearly_amount: i32,
}

#[derive(Queryable, AsChangeset, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "comments_tbl"]
pub struct Comment {
    pub comment_id: Option<i32>,
    pub applicant_id: i32,
    pub commenter: String,
    pub opinion: String,
    pub when: String,
}

#[derive(Queryable, AsChangeset, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users_tbl"]
pub struct User {
    pub user_name: String,
    pub role: String,
    pub salt: String,
    pub password: String,
}

/*
#[derive(Insertable)]
#[table_name = "applications_tbl"]
pub struct NewApplication<'a> {
    pub emp_id: i32,
    pub applicant_id: i32,
    pub name: &'a str,
    pub dob: &'a str,
    pub gender: &'a str,
    pub country: &'a str,
    pub program: &'a str,
    pub degree: &'a str,
    pub interests: &'a str,
    pub ug_university: &'a str,
    pub ug_major: &'a str,
    pub ug_degree: &'a str,
    pub ug_gpa: f64,
    pub grad_university: &'a str,
    pub grad_major: &'a str,
    pub grad_degree: &'a str,
    pub grad_gpa: f64,
    pub toefl_ielts: i32,
    pub gre: &'a str,
    pub decision: &'a str,
    pub advisor: &'a str,
    pub assistantship: &'a str,
    pub fte: f64,
    pub yearly_amount: i32,
}

// this structure contains all the fields that we can import from Slate. 
pub struct FromImport<'a> {
    pub emp_id: &'a str,
    pub applicant_id: &'a str,
    pub name: &'a str,
    pub dob: &'a str,
    pub gender: &'a str,
    pub country: &'a str,
    pub degree: &'a str,
}*/
