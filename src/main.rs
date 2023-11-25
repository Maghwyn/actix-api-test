extern crate serde_derive;

use actix_web::{App, HttpServer};

mod config;
mod cors;
mod errors;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let conf = config::get();
	let binding_address = format!("[::1]:{}", &conf.app.port);

	// log::info!("Server running on http://{}", binding_address);

	HttpServer::new(move || {
		App::new()
			.wrap(cors::options_delegate(conf.app.whitelist.clone()))
			.service(routes::module::controller())
	})
	.bind(binding_address.clone())?
	.run()
	.await
}