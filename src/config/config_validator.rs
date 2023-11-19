
use std::env;
use std::error::Error;
use dotenvy::dotenv;

pub fn env_validator() -> Result<(), Box<dyn Error>> {
	// Load environment variables from .env file.
	// Fails if .env file not found, not readable, or invalid.
	dotenv()?;

	let envs_to_check = [
		"API_PORT",
		"API_WHITELIST",
		"MONGO_URI",
		"MONGO_DBNAME",
		"JWT_SECRET",
		"COOKIE_SECURE",
		"COOKIE_SAMESITE",
	];

	// Check for missing environment variables
	let missing: Vec<_> = envs_to_check
		.iter()
		.filter_map(|&key| {
			if env::var(key).is_err() {
				Some(format!("undefined env::{}", key))
			} else {
				None
			}
		})
		.collect();

	// Throw an error if variables are missing
	if !missing.is_empty() {
		let error_message = format!("\n{}\nTrace:", missing.join("\n"));
		return Err(error_message.into());
	}

	Ok(())
}