use std::env;

// use envy;
use serde::Deserialize;
use thiserror;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub switchbot_api_token: String,
    pub switchbot_api_secret: String,
}

// #[derive(thiserror::Error, Debug)]
// pub enum ConfigError {
//     #[error("env error: {0}")]
//     EnvError(#[from] envy::Error),
// }

impl Config {
    pub fn new() -> Self {
        // Load .env file
        dotenvy::dotenv().expect(".env file not found");

        Self {
            switchbot_api_token: env::var("SWITCHBOT_API_TOKEN")
                .expect("SWITCHBOT_API_TOKEN is not set"),
            switchbot_api_secret: env::var("SWITCHBOT_API_SECRET")
                .expect("SWITCHBOT_API_SECRET is not set"),
        }
    }
}
