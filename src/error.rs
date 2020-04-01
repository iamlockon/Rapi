use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;
use derive_more::Display;

use crate::config::error::UI;

#[derive(Fail, Display, Debug)]
pub enum ApiError {
  InternalError,
  ValidationFailedError
}

impl error::ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    ResponseBuilder::new(self.status_code())
      .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
      .body(self.to_string())
  }
  fn status_code(&self) -> StatusCode {
    match *self {
      ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::ValidationFailedError => StatusCode::BAD_REQUEST
    }
  }
}
