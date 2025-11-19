use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub logging: LoggingConfig,
    pub server: ServerConfig,
}

impl Config {
    pub fn get_addr(&self) -> String {
        return format!("{}:{}", self.server.host, self.server.port);
    }
}
//Logging tool configuration
#[derive(Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub tracing_level: String,
    pub logfile_path: String,
}

impl LoggingConfig {
    //Parse the tracing level string in config file
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
