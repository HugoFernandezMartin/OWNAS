use std::sync::Arc;

use tokio::fs;

use tokio::{net::UnixListener, sync::broadcast::{self,Sender}};

use crate::file_manager::*;
use crate::ipc_handler::handle_ipc_connection;
use crate::server::Server;

pub async fn run_ipc_listener(server: Arc<Server>, tx_shutdown: Sender<()>) -> anyhow::Result<()> {
    let socket_path = "/tmp/ownas.sock";

    let workspace_path = "workspace/";
    ensure_dir(workspace_path).await?;

    //Creates unix socket
    let listener = UnixListener::bind(socket_path)?;
    tracing::info!("IPC server started");

    // Broadcast channel to send the shutdown signal through threads
    let (loop_shutdown_tx, mut loop_shutdown_rx) = broadcast::channel::<()>(1);

    loop {
        tokio::select! {
            //? REVISE check for optimization (Shared boolean?)

            //Check for shutdown signal
            _ = loop_shutdown_rx.recv() => {
                fs::remove_file(socket_path).await.ok();
                break;
            }

            //Handle IPC request
            Ok((stream, _)) = listener.accept() => {
                tracing::trace!("IPC request received");
                let loop_shutdown_tx = loop_shutdown_tx.clone();
                let server_clone = server.clone();

                tokio::spawn(handle_ipc_connection(stream, loop_shutdown_tx, server_clone, workspace_path));
            }
        } 
    }
    //Send shutdown signal to other threads
    let _ = tx_shutdown.send(());
    tracing::info!("IPC Stopped");
    Ok(())
}