use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, web, Responder, Result};

use crate::repo;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::Person;

pub fn render_400<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let json_string = serde_json::to_string(&json!({
        "message": "Bad request"
    }))
    .unwrap();

    let new_res = res.map_body(|_resphead, _respbody| {
        dev::ResponseBody::Other(dev::Body::Message(Box::new(json_string)))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn render_500<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let json_string = serde_json::to_string(&json!({
        "message": "Internal server error"
    }))
    .unwrap();

    let new_res = res.map_body(|_resphead, _respbody| {
        dev::ResponseBody::Other(dev::Body::Message(Box::new(json_string)))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}

pub async fn post_person(person: web::Json<Person>) -> impl Responder {
    format!("Hello {}! You are {}", &person.name, &person.age)
}

#[derive(Serialize, Deserialize)]
pub struct PersonId {
    value: i32,
}

pub async fn get_person(id: web::Path<PersonId>) -> impl Responder {
    repo::get_person_by_id(id.value)
}
