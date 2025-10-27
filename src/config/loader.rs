use crate::config::{Config, LoggingConfig, ServerConfig};

pub fn load_config(_config_path: &str) -> Result<Config, anyhow::Error>{
    //TODO
    Ok(Config { logging: LoggingConfig {
        tracing_level: tracing::Level::INFO
    }, server: ServerConfig {
        host: "localhost".to_string(),
        port: 8080,
        ipc_socket_path: None
    } })
}