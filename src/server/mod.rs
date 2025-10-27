pub mod builder;
pub mod ipc_listener;
pub mod tcp_listener;
pub mod handler;

use crate::config::Config;
use crate::core::errors::ServerError;
use crate::core::state::ServerStatus;

pub struct Server {
    cfg: Config,
}

impl Server {
    pub async fn stop(&self) -> Result<(), ServerError> {
        todo!()
    }
    pub fn status(&self) -> ServerStatus {
        todo!()
    }
}