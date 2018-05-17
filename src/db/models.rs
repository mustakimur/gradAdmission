use super::schema::basicinfo;

#[derive(Queryable,Debug)]
pub struct Basicinfo {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub sex: String,
    pub program: String,
    pub under_college: String,
    pub under_gpa: f32,
    pub ms_college: Option<String>,
    pub ms_gpa: Option<f32>,
    pub interests: Option<String>,
    pub decision: String,
}

#[derive(Insertable)]
#[table_name = "basicinfo"]
pub struct NewApp<'a> {
    pub id: i32,
    pub first_name: String,
    pub last_name:  &'a str,
    pub nationality:  &'a str,
    pub sex:  &'a str,
    pub program:  &'a str,
    pub under_college:  &'a str,
    pub under_gpa: f32,
    pub ms_college: Option< &'a str>,
    pub ms_gpa: Option<f32>,
    pub interests:  &'a str,
    pub decision:  &'a str,
}