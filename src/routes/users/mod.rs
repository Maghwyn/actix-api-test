pub mod users_controller;
pub mod users_service;

pub mod module {
	use actix_web::{web, Scope};
	use super::users_controller::*;

	pub fn controller() -> Scope {
		web::scope("/users")
			.configure(|cfg: &mut web::ServiceConfig| {
				cfg.service(get_me);
			})
	}
}