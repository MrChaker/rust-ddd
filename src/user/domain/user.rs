use super::object_values::course::Course;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: String,
    name: String,
    course: Course,
}
