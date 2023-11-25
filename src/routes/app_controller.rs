use actix_web::{HttpResponse, Responder, get};

use crate::errors::http_errors::ServiceError;
use crate::routes::app_service as app_service;

#[get("/")]
async fn hello() -> Result<impl Responder, ServiceError> {
	let hello_world = app_service::hello_world();
	Ok(HttpResponse::Ok().body(hello_world))
}