#[derive(Debug)]
pub struct EnvConfiguration {
	pub app: AppConfig,
	pub mongo: MongoConfig,
	pub jwt: JwtConfig,
}

#[derive(Debug)]
pub struct AppConfig {
	pub port: Option<String>,
	pub env: Option<String>,
	pub whitelist: Vec<String>,
}

#[derive(Debug)]
pub struct MongoConfig {
	pub uri: Option<String>,
	pub dbname: Option<String>,
}

#[derive(Debug)]
pub struct JwtConfig {
	pub secret: Option<String>,
	pub cookie: JwtCookieConfig,
}

#[derive(Debug)]
pub struct JwtCookieConfig {
	pub secure: Option<bool>,
	pub samesite: Option<String>,
}