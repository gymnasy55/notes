use actix_web::{
    body::BoxBody,
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum UserError {
    NotFound,
    #[allow(dead_code)]
    CreationFailed,
    #[allow(dead_code)]
    BadRequest,
    InternalError,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::NotFound => StatusCode::NOT_FOUND,
            UserError::CreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::BadRequest => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
