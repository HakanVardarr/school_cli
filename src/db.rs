use crate::models::NewStudent;

use super::models::Student;
use super::schema::students;
use super::schema::students::dsl::*;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub struct Db {
    connection: PgConnection,
}

impl Db {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("ERROR: No database url found on .env file");

        let connection = PgConnection::establish(&database_url)
            .expect(&format!("ERROR: cannot connect to {}", database_url));

        Self { connection }
    }
    pub fn show_students(&mut self) {
        let results = students
            .load::<Student>(&mut self.connection)
            .expect("ERROR: loading students");

        if results.len() > 0 {
            println!("---------------------");
        }
        for student in results {
            println!(
                "{}) {} {} {}",
                student.id, student.name, student.surname, student.age
            );
            println!("---------------------");
        }
    }

    pub fn add_student(&mut self, n: &str, s: &str, a: i16, g: f32) {
        if g > 5.0 {
            eprintln!("ERROR: GPA cannot be greater than 5.0");

            return;
        }

        let new_student = NewStudent {
            name: n,
            surname: s,
            age: a,
            gpa: g,
        };

        diesel::insert_into(students::table)
            .values(&new_student)
            .execute(&mut self.connection)
            .expect("ERROR: saving new student");
    }

    pub fn get_students_gpa(&mut self, id: i32) {
        let student = students
            .filter(student_id.eq(id))
            .load::<Student>(&mut self.connection)
            .expect("ERROR: loading students");

        for s in student {
            println!("{} {}'s GPA is {}", s.name, s.surname, s.gpa);
        }
    }

    pub fn delete_student(&mut self, id: i32) {
        diesel::delete(students.find(id))
            .execute(&mut self.connection)
            .expect("ERROR: deleting student");
    }
}
