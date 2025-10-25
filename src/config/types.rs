#[derive(Clone)]
pub struct Config {
    pub logging: LoggingConfig,
    pub server: ServerConfig
}

//Logging tool configuration
#[derive(Clone)]
pub struct LoggingConfig {
    pub tracing_level: tracing::Level
}

//Server configuration
#[derive(Clone)]
pub struct ServerConfig {
    pub addr: String
}