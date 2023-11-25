use actix_web::{post, HttpRequest, HttpResponse, Responder};

use crate::errors::http_errors::ServiceError;
use crate::routes::auth::auth_service as auth_service;

#[post("/")]
async fn is_auth(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::check_auth();
	Ok(HttpResponse::Ok().finish())
}

#[post("/signup")]
async fn signup(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::signup();
	Ok(HttpResponse::Created().finish())
}

#[post("/signin")]
async fn signin(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::validate_user();
	auth_service::generate_token();
	Ok(HttpResponse::Ok().finish())
}

#[post("/logout")]
async fn logout() -> Result<impl Responder, ServiceError> {
	// logic here
	Ok(HttpResponse::Ok().finish())
}

#[post("/ask-activation-token")]
async fn ask_activation_token(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::ask_activation_token();
	Ok(HttpResponse::Created().finish())
}

#[post("/ask-reset-token")]
async fn ask_reset_token(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::ask_reset_pwd_token();
	Ok(HttpResponse::Created().finish())
}

#[post("/activate")]
async fn activate_account(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::activate_account();
	auth_service::generate_token();
	Ok(HttpResponse::Ok().finish())
}

#[post("/reset-password")]
async fn reset_password(_req: HttpRequest) -> Result<impl Responder, ServiceError> {
	auth_service::reset_password();
	Ok(HttpResponse::Ok().finish())
}