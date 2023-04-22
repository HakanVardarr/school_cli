use super::schema::students;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub age: i16,
    pub gpa: f32,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
    pub name: &'a str,
    pub surname: &'a str,
    pub age: i16,
    pub gpa: f32,
}
