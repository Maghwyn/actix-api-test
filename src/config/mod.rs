use std::env;
use std::error::Error;
use once_cell::sync::Lazy;

mod config_struct;
mod config_validator;

use self::config_struct::*;
use self::config_validator::env_validator;

impl EnvConfiguration {
	pub fn load() -> Result<Self, Box<dyn Error>> {
		if let Err(err) = env_validator() {
			// Would result in undefined behavior or an unreliable state of the application
			panic!("Failed to validate environment variables: {}", err);
		}

		Ok(EnvConfiguration {
			app: AppConfig {
				port: env::var("API_PORT").unwrap(),
				env: env::var("RUST_ENV").unwrap(),
				whitelist: match serde_json::from_str(&env::var("API_WHITELIST").unwrap()) {
					Ok(parsed_array) => parsed_array,
					Err(err) => {
						// Would result in undefined behavior or an unreliable state of the application
						panic!("Error parsing API_WHITELIST as JSON: {}", err);
					}
				}
			},
			mongo: MongoConfig {
				uri: env::var("MONGO_URI").unwrap(),
				dbname: env::var("MONGO_DBNAME").unwrap(),
			},
			jwt: JwtConfig {
				secret: env::var("JWT_SECRET").unwrap(),
				cookie: JwtCookieConfig {
					secure: env::var("COOKIE_SECURE").unwrap().parse().unwrap(),
					samesite: env::var("COOKIE_SAMESITE").unwrap(),
				},
			},
		})
	}
}

static CONFIG: Lazy<EnvConfiguration> = Lazy::new(|| {
	EnvConfiguration::load().unwrap_or_else(|err| {
		// Would result in undefined behavior or an unreliable state of the application
		panic!("Failed to load configuration: {}", err);
	})
});

// The environement configuration is not mutable
pub fn get() -> &'static EnvConfiguration {
	return &*CONFIG
}