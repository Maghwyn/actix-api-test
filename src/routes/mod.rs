pub mod app_controller;
pub mod app_service;

pub mod auth;
pub mod users;

pub mod module {
	use actix_web::{web, Scope};
	use super::app_controller::*;
	use super::auth;
	use super::users;

	pub fn controller() -> Scope {
		web::scope("/")
			.configure(|cfg: &mut web::ServiceConfig| {
				cfg.service(hello);

				cfg.service(auth::module::controller());
				cfg.service(users::module::controller());
			})
	}
}