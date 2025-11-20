mod control;

use std::sync::Arc;

use axum::Router;
use tokio::sync::mpsc::Sender;

use crate::{Server, control::ControlSignal};

#[derive(Clone)]
struct ServerState {
    server: Arc<Server>,
    control_tx: Sender<ControlSignal>,
}

pub fn create_service(server: Arc<Server>, control_tx: Sender<ControlSignal>) -> Router {
    Router::new()
        .nest("/", control::router())
        .with_state(ServerState {
            server: server.clone(),
            control_tx,
        })
}
