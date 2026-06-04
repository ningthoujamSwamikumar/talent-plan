use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub features: FeatureFlags,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub shutdown_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeatureFlags {
    pub enable_search: bool,
    pub enable_websockets: bool,
    pub max_tasks_per_project: usize,
}

impl AppConfig {
    /// Load configuration with the following precedence (last wins):
    /// 1. config/default.toml
    /// 2. config/{APP_ENV}.toml (if APP_ENV is set)
    /// 3. Environment variables prefixed with APP_ (nested via __)
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            // Layer 1: compiled defaults from file
            .add_source(File::with_name("config/default").required(true));

        // Layer 2: environment-specific overlay
        if let Ok(env_name) = env::var("APP_ENV") {
            builder = builder.add_source(
                File::with_name(&format!("config/{}", env_name)).required(false),
            );
        }

        // Layer 3: environment variables — APP_SERVER__PORT=8080 etc.
        builder = builder.add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true),
        );

        let config = builder.build()?;
        let app_config: AppConfig = config.try_deserialize()?;

        // Validation
        if app_config.server.port == 0 {
            return Err(ConfigError::Message(
                "server.port must be greater than 0".into(),
            ));
        }
        if app_config.database.max_connections < 1 {
            return Err(ConfigError::Message(
                "database.max_connections must be at least 1".into(),
            ));
        }
        if app_config.features.max_tasks_per_project < 1 {
            return Err(ConfigError::Message(
                "features.max_tasks_per_project must be at least 1".into(),
            ));
        }

        Ok(app_config)
    }
}
