use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:data.db".to_string()),
            openrouter_api_key: env::var("OPENROUTER_API_KEY")
                .unwrap_or_default(),
            openrouter_model: env::var("OPENROUTER_API_MODEL")
                .expect("OPENROUTER_API_MODEL must be set"),
            api_key: env::var("API_KEY")
                .unwrap_or_default(),
        }
    }
}
