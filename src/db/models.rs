use super::schema::ApplicationsTbl;
use super::schema::CommentsTbl;
use super::schema::UsersTbl;

#[derive(Queryable, Debug)]
pub struct Application {
    pub EmpID: i32,
    pub ApplicantID: i32,
    pub Name: String,
    pub DOB: String,
    pub Gender: String,
    pub Country: String,
    pub Program: String,
    pub Degree: String,
    pub Interests: String,
    pub UG_University: String,
    pub UG_Major: String,
    pub UG_Degree: String,
    pub UG_GPA: f32,
    pub Grad_University: String,
    pub Grad_Major: String,
    pub Grad_Degree: String,
    pub Grad_GPA: f32,
    pub TOEFL_IELTS: i32,
    pub GRE_Verb: i32,
    pub GRE_Quanti: i32,
    pub GRE_Combined: i32,
    pub Decision: String,
    pub Advisor: String,
    pub Assistantship: String,
    pub FTE: f32,
    pub YearlyAmount: i32,
}

#[derive(Queryable, Debug)]
pub struct Comment {
    pub CommentId: i32,
    pub EmpID: i32,
    pub Commenter: i32,
    pub Opinion: i32,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub Username: String,
    pub Role: String,
    pub Password: String,
}

#[derive(Insertable)]
#[table_name = "ApplicationsTbl"]
pub struct NewApplication {
    pub EmpID: i32,
    pub ApplicantID: i32,
    pub Name: String,
    pub DOB: String,
    pub Gender: String,
    pub Country: String,
    pub Program: String,
    pub Degree: String,
    pub Interests: String,
    pub UG_University: String,
    pub UG_Major: String,
    pub UG_Degree: String,
    pub UG_GPA: f32,
    pub Grad_University: String,
    pub Grad_Major: String,
    pub Grad_Degree: String,
    pub Grad_GPA: f32,
    pub TOEFL_IELTS: i32,
    pub GRE_Verb: i32,
    pub GRE_Quanti: i32,
    pub GRE_Combined: i32,
    pub Decision: String,
    pub Advisor: String,
    pub Assistantship: String,
    pub FTE: f32,
    pub YearlyAmount: i32,
}

