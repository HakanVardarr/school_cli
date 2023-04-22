CREATE TABLE students (
    student_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    surname VARCHAR(50) NOT NULL,
    age SMALLINT NOT NULL,
    gpa REAL NOT NULL
)