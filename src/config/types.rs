
#[derive(Clone)]
pub struct Config {
    pub logging: LoggingConfig,
    pub server: ServerConfig
}


impl Config {
    pub fn ipc_socket(&self) -> &str {
        self.server.ipc_socket_path.as_deref().unwrap_or("/tmp/ownas.sock")
    }
}

//Logging tool configuration
#[derive(Clone)]
pub struct LoggingConfig {
    pub tracing_level: tracing::Level
}

//Server configuration
#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub ipc_socket_path: Option<String>
}

impl ServerConfig {
    pub fn get_addr(&self) -> String {
        return format!("{}:{}", self.host, self.port);
    }
}