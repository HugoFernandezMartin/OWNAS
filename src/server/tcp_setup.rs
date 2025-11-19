use std::{net::SocketAddr, str::FromStr, sync::Arc};

use tokio::{
    net::TcpListener,
    sync::{broadcast::Receiver, mpsc::Sender},
};

use crate::{Server, control::ControlSignal, routes::create_service};

pub async fn run_tcp_server(
    server: Arc<Server>,
    control_tx: Sender<ControlSignal>,
    mut rx_shutdown: Receiver<()>,
) -> anyhow::Result<()> {
    tracing::debug!("Trying to start TCP server");

    //Parse addr
    let str_addr = &server.cfg.get_addr();
    tracing::trace!("TCP addr: {str_addr}");
    let addr = match SocketAddr::from_str(str_addr) {
        Ok(addr) => addr,
        Err(e) => {
            tracing::error!(error = %e, "Unable to parse addr");
            anyhow::bail!("Unable to parse addr")
        }
    };

    let app = create_service(server, control_tx);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Started tcp server in http://{}", addr);

    //Iniciate axum server. It will stop if it broadcast receive shutdown signal
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = rx_shutdown.recv().await;
            tracing::info!("TCP server shutting down");
        })
        .await?;

    Ok(())
}
