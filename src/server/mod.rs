pub mod builder;
pub mod ipc_listener;
pub mod tcp_listener;
pub mod handler;

use std::time::Instant;

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
}