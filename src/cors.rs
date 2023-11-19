use actix_cors::Cors;

pub fn options_delegate(whitelist: Vec<String>) -> Cors {
	Cors::default()
		.allowed_origin_fn(move |origin, _req_head| {
			let str_origin = origin.to_str().unwrap();
			whitelist.iter().any(|r| r.as_str() == str_origin)
		})
		.allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
		.allowed_headers(vec![
			actix_web::http::header::AUTHORIZATION,
			actix_web::http::header::CONTENT_TYPE,
			actix_web::http::header::ACCEPT,
		])
		.expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
		.supports_credentials()
		.max_age(3600)
}