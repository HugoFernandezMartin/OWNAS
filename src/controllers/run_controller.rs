use std::sync::Arc;

use crate::{
    Server,
    core::responses::{ResponseType, ServerResponse},
};

pub async fn show_log_controller(server_clone: Arc<Server>) -> ServerResponse {
    return match server_clone.get_log().await {
        Ok(log) => ServerResponse::Success(ResponseType::Info(log)),
        Err(e) => {
            tracing::error!(error = %e, "Unable to get log");
            ServerResponse::Error(e.to_string())
        }
    };
}
