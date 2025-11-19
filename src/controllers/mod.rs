use tokio::sync::mpsc::Sender;

use crate::{
    control::ControlSignal,
    core::responses::{ResponseType, ServerResponse},
};

pub mod file_controller;
pub mod run_controller;

pub async fn stop_controller(control_tx: Sender<ControlSignal>) -> ServerResponse {
    //Send shutdown signal to the father
    return match control_tx.send(ControlSignal::Shutdown).await {
        Ok(_) => {
            ServerResponse::Success(ResponseType::Info("Server stopped succesfully".to_string()))
        }
        Err(e) => {
            tracing::error!(error = %e, "Unable to send shutdown signal");
            ServerResponse::Error(e.to_string())
        }
    };
}

pub async fn restart_controller(control_tx: Sender<ControlSignal>) -> ServerResponse {
    //Send shutdown signal to the father
    return match control_tx.send(ControlSignal::Restart).await {
        Ok(_) => ServerResponse::Success(ResponseType::Info(
            "Server restarted succesfully".to_string(),
        )),
        Err(e) => {
            tracing::error!(error = %e, "Unable to send restart signal");
            ServerResponse::Error(e.to_string())
        }
    };
}
