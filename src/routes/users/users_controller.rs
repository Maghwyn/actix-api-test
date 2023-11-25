use actix_web::{get, HttpResponse, Responder};

use crate::errors::http_errors::ServiceError;
use crate::routes::users::users_service as users_service;

#[get("/me")]
async fn get_me() -> Result<impl Responder, ServiceError> {
	let user = users_service::retrieve_general_information();
	Ok(HttpResponse::Ok().body(user))
}