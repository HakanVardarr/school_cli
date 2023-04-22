// @generated automatically by Diesel CLI.

diesel::table! {
    students (student_id) {
        student_id -> Int4,
        name -> Varchar,
        surname -> Varchar,
        age -> Int2,
        gpa -> Float4,
    }
}
