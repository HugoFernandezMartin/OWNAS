use axum::{Json, Router, extract::State, routing::get};

use crate::{
    controllers::{restart_controller, stop_controller},
    core::responses::{ResponseType, ServerResponse},
    routes::ServerState,
};

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/status", get(get_status))
        .route("/restart", get(restart_server))
        .route("/stop", get(stop_server))
}

async fn get_status(state: State<ServerState>) -> Json<ServerResponse> {
    let status = state.server.get_status();
    Json(ServerResponse::Success(ResponseType::Status(status)))
}

async fn restart_server(state: State<ServerState>) -> Json<ServerResponse> {
    let response = restart_controller(state.control_tx.clone()).await;
    Json(response)
}

async fn stop_server(state: State<ServerState>) -> Json<ServerResponse> {
    let response = stop_controller(state.control_tx.clone()).await;
    Json(response)
}
