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

pub enum Reference {
    Reference,
}

use std::convert::Into;

impl Into<Reference> for i32 {
    fn into(self) -> Reference {
        Reference::Reference
    }
}

impl Into<Reference> for String {
    fn into(self) -> Reference {
        Reference::Reference
    }
}

pub trait FromDb<T>
where
    T: Into<Reference>,
{
    fn from_db(conn: Connection, reference: T) -> rusqlite::Result<Self>
    where
        Self: Sized;
}
