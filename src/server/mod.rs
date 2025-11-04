pub mod builder;
pub mod ipc_listener;
pub mod ipc_handler;
pub mod tcp_listener;
pub mod tcp_handler;
pub mod file_manager;

use std::{time::Instant};

use tokio::fs;

use crate::{config::Config, core::state::{ServerStatus, Status}};
pub struct Server {
    cfg: Config,
    data: ServerData
}

pub struct ServerData {
    start_time: Instant
}

impl Server {
    pub fn get_status(&self) -> ServerStatus {
        ServerStatus::new(Status::Running, std::process::id(), self.data.start_time, &self.cfg)
    }

    pub async fn get_log(&self) -> Result<String, anyhow::Error> {
        let file = fs::read(&self.cfg.logging.logfile_path).await?;
        let log = String::from_utf8(file)?;
        Ok(log)
    }

}