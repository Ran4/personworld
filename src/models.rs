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
