diesel::table! {
    students_with_points (student_id) {
        student_id -> Integer,
        name -> Text,
        grade -> Integer,
        point_value -> Integer,
    }
}