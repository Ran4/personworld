use crate::models::Person;

pub fn get_person_by_id(id: i32) -> Person {
    Person {
        name: String::from("Ex"),
        age: id,
    }
}
