use crate::config::{Config, LoggingConfig, ServerConfig};

pub fn load_config(config_path: &str) -> Result<Config, String>{
    Ok(Config { logging: LoggingConfig {
        tracing_level: tracing::Level::INFO
    }, server: ServerConfig {
        addr: "127.0.0.1:8080".to_string()
    } })
}