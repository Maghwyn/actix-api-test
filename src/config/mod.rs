use std::env;
use std::error::Error;
use once_cell::sync::Lazy;

mod config_struct;
mod config_validator;

use self::config_struct::*;
use self::config_validator::env_validator;

impl EnvConfiguration {
	pub fn load() -> Result<Self, Box<dyn Error>> {
		println!("Load function is being called");

		if let Err(err) = env_validator() {
			// Would result in undefined behavior or an unreliable state of the application
			panic!("Failed to validate environment variables: {}", err);
		}

		Ok(EnvConfiguration {
			app: AppConfig {
				port: env::var("API_PORT").ok(),
				env: env::var("NODE_ENV").ok(),
				whitelist: serde_json::from_str(&env::var("API_WHITELIST")
					.unwrap_or("[]".to_string()))
					.unwrap_or_else(|_| vec![env::var("API_WHITELIST").unwrap_or_default()]),
			},
			mongo: MongoConfig {
				uri: env::var("MONGO_URI").ok(),
				dbname: env::var("MONGO_DBNAME").ok(),
			},
			jwt: JwtConfig {
				secret: env::var("JWT_SECRET").ok(),
				cookie: JwtCookieConfig {
					secure: env::var("COOKIE_SECURE").ok()
						.and_then(|s| s.parse().ok()),
					samesite: env::var("COOKIE_SAMESITE").ok(),
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