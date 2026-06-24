use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    status_code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display)]
pub enum CustomError {
    #[display("There is some internal server error")]
    InternalError,
    #[display("User not found")]
    UserNotFound,
    #[display("Please Login First")]
    LoginFirst,
    #[display("Please Provide jwt token")]
    JwtMissing,
    #[display("User already Exist")]
    UserExists,
    #[display("Wrong password")]
    WrongPassword,
    #[display("wrong jwt token")]
    WrongJwtToken,
}

impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::UserNotFound => StatusCode::UNAUTHORIZED,
            CustomError::JwtMissing => StatusCode::UNAUTHORIZED,
            CustomError::LoginFirst => StatusCode::UNAUTHORIZED,
            CustomError::UserExists => StatusCode::UNAUTHORIZED,
            CustomError::WrongPassword => StatusCode::BAD_REQUEST,
            CustomError::WrongJwtToken => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();

        HttpResponse::build(status_code).json(ErrorResponse {
            status_code: status_code.as_u16(),
            error: status_code
                .canonical_reason()
                .unwrap_or("unknown")
                .to_string(),
            message: self.to_string(),
        })
    }
}
