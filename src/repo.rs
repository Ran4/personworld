use crate::models::Person;
use rusqlite::{params, Connection};

pub fn get_conn() -> Connection {
    let conn = Connection::open_in_memory().expect("Could not open sqlite conn");

    conn.execute(
        "CREATE TABLE person (name TEXT NOT NULL, age  INTEGER NOT NULL)",
        params![],
    )
    .expect("Create table sql execution failed");

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params!["John Johnsson", 43],
    )
    .expect("Inserting person into sqlite failed");

    conn
}

pub fn get_person_by_id(conn: Connection, id: i32) -> rusqlite::Result<Person> {
    conn.query_row(
        "SELECT name, age FROM person WHERE age = ?1",
        params![id],
        |row| {
            Ok(Person {
                name: row.get(0)?,
                age: row.get(1)?,
            })
        },
    )
}
