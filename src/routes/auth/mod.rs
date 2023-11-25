pub mod auth_controller;
pub mod auth_service;

pub mod module {
	use actix_web::{web, Scope};
	use super::auth_controller::*;

	pub fn controller() -> Scope {
		web::scope("/auth")
			.configure(|cfg: &mut web::ServiceConfig| {
				cfg.service(is_auth);
				cfg.service(signup);
				cfg.service(signin);
				cfg.service(logout);
				cfg.service(ask_activation_token);
				cfg.service(ask_reset_token);
				cfg.service(activate_account);
				cfg.service(reset_password);
			})
	}
}