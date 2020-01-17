use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

fn json_response(x: impl Serialize) -> Ready<Result<HttpResponse, Error>> {
    let body = serde_json::to_string(&x).unwrap();

    ready(Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body)))
}

impl Responder for Person {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        json_response(self)
    }
}

use crate::repo::FromDb;
use rusqlite::{params, Connection};

impl FromDb<i32> for Person {
    fn from_db(conn: Connection, reference: i32) -> rusqlite::Result<Self> {
        conn.query_row(
            "SELECT name, age FROM person WHERE age = ?1",
            params![reference],
            |row| {
                Ok(Person {
                    name: row.get(0)?,
                    age: row.get(1)?,
                })
            },
        )
    }
}

impl FromDb<String> for Person {
    fn from_db(conn: Connection, reference: String) -> rusqlite::Result<Self> {
        conn.query_row(
            "SELECT name, age FROM person WHERE name = ?1",
            params![reference],
            |row| {
                Ok(Person {
                    name: row.get(0)?,
                    age: row.get(1)?,
                })
            },
        )
    }
}
