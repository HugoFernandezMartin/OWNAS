use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use tokio::sync::mpsc::Sender;

use crate::{
    Server,
    control::ControlSignal,
    controllers::stop_controller,
    core::responses::{ResponseType, ServerResponse},
};

#[derive(Clone)]
struct ServerState {
    server: Arc<Server>,
    control_tx: Sender<ControlSignal>,
}

pub fn create_service(server: Arc<Server>, control_tx: Sender<ControlSignal>) -> Router {
    Router::new()
        .route("/status", get(get_status))
        .route("/restart", get(restart_server))
        .route("/stop", get(stop_server))
        .with_state(ServerState {
            server: server.clone(),
            control_tx,
        })
}

async fn get_status(state: State<ServerState>) -> Json<ServerResponse> {
    let status = state.server.get_status();
    Json(ServerResponse::Success(ResponseType::Status(status)))
}

async fn restart_server(_state: State<ServerState>) -> Json<ServerResponse> {
    todo!() //Check with chat
}

async fn stop_server(state: State<ServerState>) -> Json<ServerResponse> {
    let response = stop_controller(state.control_tx.clone()).await;
    Json(response)
}
