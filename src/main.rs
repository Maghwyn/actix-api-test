extern crate serde_derive;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod config;
mod cors;
mod errors;

use errors::http_errors::ServiceError;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[get("/test")]
async fn echo() -> Result<impl Responder, ServiceError> {
	if true {
		return Err::<HttpResponse, ServiceError>(ServiceError::Forbidden("Some error message".to_string()))
	}

	Ok(HttpResponse::Ok().body("Hey there!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let conf = config::get();
	let binding_address = format!("[::1]:{}", &conf.app.port);

	HttpServer::new(|| {
		App::new()
			.wrap(cors::options_delegate(conf.app.whitelist.clone()))
			.service(hello)
			.service(echo)
	})
	.bind(binding_address)?
	.run()
	.await
}