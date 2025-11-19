use serde::{Deserialize, Serialize};

use crate::core::state::ServerStatus;

#[derive(Serialize, Deserialize)]
pub enum ServerResponse {
    Success(ResponseType),
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum ResponseType {
    Status(ServerStatus),
    Files(Vec<String>),
    Info(String),
}
