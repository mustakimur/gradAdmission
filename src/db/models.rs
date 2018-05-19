use super::schema::ApplicationsTbl;
use super::schema::CommentsTbl;
use super::schema::UsersTbl;

#[derive(Queryable, Debug)]
pub struct Application {
    pub emp_id: i32,
    pub applicant_id: i32,
    pub name: String,
    pub dob: String,
    pub gender: String,
    pub country: String,
    pub program: String,
    pub degree: String,
    pub interests: String,
    pub ug_university: String,
    pub ug_major: String,
    pub ug_degree: String,
    pub ug_gpa: f32,
    pub grad_university: String,
    pub grad_major: String,
    pub grad_degree: String,
    pub grad_gpa: f32,
    pub toefl_ielts: i32,
    pub gre_verb: i32,
    pub gre_quanti: i32,
    pub gre_combined: i32,
    pub decision: String,
    pub advisor: String,
    pub assistantship: String,
    pub fte: f32,
    pub yearly_amount: i32,
}

#[derive(Queryable, Debug)]
pub struct Comment {
    pub comment_id: i32,
    pub emp_id: i32,
    pub commenter: i32,
    pub opinion: i32,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub user_name: String,
    pub role: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "ApplicationsTbl"]
pub struct NewApplication {
    pub emp_id: i32,
    pub applicant_id: i32,
    pub name: String,
    pub dob: String,
    pub gender: String,
    pub country: String,
    pub program: String,
    pub degree: String,
    pub interests: String,
    pub ug_university: String,
    pub ug_major: String,
    pub ug_degree: String,
    pub ug_gpa: f32,
    pub grad_university: String,
    pub grad_major: String,
    pub grad_degree: String,
    pub grad_gpa: f32,
    pub toefl_ielts: i32,
    pub gre_verb: i32,
    pub gre_quanti: i32,
    pub gre_combined: i32,
    pub decision: String,
    pub advisor: String,
    pub assistantship: String,
    pub fte: f32,
    pub yearly_amount: i32,
}

