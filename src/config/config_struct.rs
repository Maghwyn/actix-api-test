#[derive(Debug)]
pub struct EnvConfiguration {
	pub app: AppConfig,
	pub mongo: MongoConfig,
	pub jwt: JwtConfig,
}

#[derive(Debug)]
pub struct AppConfig {
	pub port: String,
	pub env: String,
	pub whitelist: Vec<String>,
}

#[derive(Debug)]
pub struct MongoConfig {
	pub uri: String,
	pub dbname: String,
}

#[derive(Debug)]
pub struct JwtConfig {
	pub secret: String,
	pub cookie: JwtCookieConfig,
}

#[derive(Debug)]
pub struct JwtCookieConfig {
	pub secure: bool,
	pub samesite: String,
}