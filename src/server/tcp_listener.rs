use std::sync::Arc;

use tokio::{net::TcpListener, sync::broadcast::Receiver};

use crate::server::{Server, handler::handle_connection};



pub async fn run_tcp_listener(server: Arc<Server>, mut rx_shutdown: Receiver<()>) -> anyhow::Result<()> {
    let addr = server.cfg.server.get_addr();
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("ErrorTCP: {}", e);
            return anyhow::Ok(());
        }
    };
    tracing::info!("Server TCP started on {}", &addr);
    loop {
        tokio::select! {
            Ok((socket, _)) = listener.accept() => {
                tokio::spawn(handle_connection(socket));
            }
            _ = rx_shutdown.recv() => {
                tracing::info!("TCP listener stopped successfully");
                break;
            }
        }
    }  
    Ok(())
}