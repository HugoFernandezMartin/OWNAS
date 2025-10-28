use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub logging: LoggingConfig,
    pub server: ServerConfig
}

//Logging tool configuration
#[derive(Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub tracing_level: String,
    pub logfile_path: String
}

impl LoggingConfig {
    pub fn get_tracing_level(&self) -> tracing::Level {
        match self.tracing_level.as_str() {
            "INFO" => tracing::Level::INFO,
            "DEBUG" => tracing::Level::DEBUG,
            "ERROR" => tracing::Level::ERROR,
            "TRACE" => tracing::Level::TRACE,
            "WARN" => tracing::Level::WARN,
            invalid => {
                eprintln!("⚠️ Invalid tracing level '{}', defaulting to INFO", invalid);
                tracing::Level::INFO
            }
        }
    }
}

//Server configuration
#[derive(Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_addr(&self) -> String {
        return format!("{}:{}", self.host, self.port);
    }
}