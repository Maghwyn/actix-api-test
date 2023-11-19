use std::fmt;
use actix_web::HttpResponse;
use serde_derive::Serialize;

#[derive(Debug)]
pub enum ServiceError {
	BadRequest(String),
	Unauthorized(String),
	Forbidden(String),
	NotFound(String),
	InternalServerError(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
	pub message: String,
	pub error_type: String,
	pub time: String,
}

impl fmt::Display for ServiceError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ServiceError::BadRequest(error)
			| ServiceError::Unauthorized(error)
			| ServiceError::Forbidden(error)
			| ServiceError::NotFound(error)
			| ServiceError::InternalServerError(error) => write!(f, "{}", error),
		}
	}
}

impl actix_web::error::ResponseError for ServiceError {
	fn error_response(&self) -> HttpResponse {
		let (status, default_message) = match self {
			ServiceError::BadRequest(_) => (actix_web::http::StatusCode::BAD_REQUEST, "Bad request"),
			ServiceError::Unauthorized(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "Unauthorized"),
			ServiceError::Forbidden(_) => (actix_web::http::StatusCode::FORBIDDEN, "Forbidden"),
			ServiceError::NotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "Not Found"),
			ServiceError::InternalServerError(_) => (
				actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
				"Internal server error",
			),
		};

		let error_message = match self {
			ServiceError::BadRequest(error)
			| ServiceError::Unauthorized(error)
			| ServiceError::Forbidden(error)
			| ServiceError::NotFound(error)
			| ServiceError::InternalServerError(error) => error.clone(),
		};

		HttpResponse::build(status).json(ErrorResponse {
			message: error_message,
			error_type: default_message.to_string(),
			time: chrono::Utc::now().to_string(),
		})
	}

	fn status_code(&self) -> actix_web::http::StatusCode {
		actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
	}
}
