use std::sync::Arc;

use tokio::{net::TcpListener, sync::broadcast::Receiver};

use crate::server::{Server, tcp_handler::handle_tcp_connection};

pub async fn run_tcp_listener(
    server: Arc<Server>,
    mut rx_shutdown: Receiver<()>,
) -> anyhow::Result<()> {
    let addr = server.cfg.server.get_addr();
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!(error = %e, "Unable to start TCP Listener");
            anyhow::bail!("Unable to start TCP Listener: {e}");
        }
    };
    tracing::info!(?addr, "TCP Listener started successfully");
    loop {
        tokio::select! {
            Ok((socket, _)) = listener.accept() => {
                tokio::spawn(handle_tcp_connection(socket));
            }
            _ = rx_shutdown.recv() => {
                tracing::info!("TCP listener stopped successfully");
                break;
            }
        }
    }
    Ok(())
}
