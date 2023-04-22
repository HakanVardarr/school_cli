use clap::{Parser, Subcommand};
mod db;
mod models;
mod schema;

pub use db::Db;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();
        let mut db = Db::new();

        match &cli.command {
            Commands::Create {
                name,
                surname,
                age,
                gpa,
            } => db.add_student(name, surname, *age, *gpa),
            Commands::ShowGPA { id } => db.get_students_gpa(*id),
            Commands::ShowAll => db.show_students(),
            Commands::Delete { id } => db.delete_student(*id),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Creates student. 'USAGE: <program> create NAME SURNAME AGE GPS'
    Create {
        name: String,
        surname: String,
        age: i16,
        gpa: f32,
    },
    /// Deletes the student. 'USAGE: <program> delete ID'
    Delete { id: i32 },
    /// Shows the GPA of given students id. 'USAGE: <program> show-gpa ID'
    ShowGPA { id: i32 },
    /// Shows all the recorded students without their GPAs. 'USAGE: <program> show-all'
    ShowAll,
}
