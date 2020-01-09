use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;
use serde_json::json;

#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Not Found")]
    NotFound,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let json_string = serde_json::to_string(&json!({
            "message": self.to_string()
        }))
        .expect("Programmer error: invalid JSON");

        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(json_string)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
