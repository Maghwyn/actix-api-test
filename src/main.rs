extern crate serde_derive;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod config;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let conf = config::get();
	let binding_address = format!("[::1]:{}", &conf.app.port);

	HttpServer::new(|| {
		App::new()
			.service(hello)
			.service(echo)
			.route("/hey", web::get().to(manual_hello))
	})
	.bind(binding_address)?
	.run()
	.await
}