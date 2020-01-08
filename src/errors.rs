use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};

use failure::Fail;

#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Not Found")]
    NotFound,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
