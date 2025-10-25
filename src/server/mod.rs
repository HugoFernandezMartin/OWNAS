pub mod builder;



use tokio::net::TcpListener;

use crate::config::Config;
use crate::core::errors::ServerError;
use crate::core::state::ServerStatus;
pub struct Server {
    cfg: Config,
}

impl Server {
    pub async fn start(&self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(&self.cfg.server.addr).await?;
        tracing::info!("Server listening on {}", self.cfg.server.addr);
        loop {
            match listener.accept().await {
            Ok((socket, client_addr)) => {
                tracing::info!("Connected client from: {client_addr}");
            }
            Err(e) => {
                tracing::error!("Unable to connect to client: {e}");
            }
        }
        }
    }
    
    pub async fn stop(&self) -> Result<(), ServerError> {
        todo!()
    }
    pub fn status(&self) -> ServerStatus {
        todo!()
    }
}