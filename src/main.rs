use school_db::Db;

fn main() {
    let mut db = Db::new();
    db.get_students_gpa(1);
}
