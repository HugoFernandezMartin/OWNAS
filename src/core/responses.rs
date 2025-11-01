use serde::{Deserialize, Serialize};

use crate::core::state::ServerStatus;

#[derive(Serialize, Deserialize)]
pub enum DaemonResponse {
    Status(ServerStatus),
    Info(String),
    Error(String)
}