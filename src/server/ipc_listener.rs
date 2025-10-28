use std::sync::Arc;

use tokio::fs;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::UnixListener, sync::broadcast::{self,Sender}};

use crate::Server;

pub async fn run_ipc_listener(server: Arc<Server>, tx_shutdown: Sender<()>) -> anyhow::Result<()> {
    let socket_path = server.cfg.ipc_socket();
    //Creates unix socket
    let listener = UnixListener::bind(socket_path)?;
    tracing::info!("IPC server running");

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
            Ok((mut stream, _)) = listener.accept() => {
                let loop_shutdown_tx = loop_shutdown_tx.clone();

                tokio::spawn(async move {
                    let mut buffer = vec![0; 1024];
                    let n = stream.read(&mut buffer).await?;
                    let message = String::from_utf8_lossy(&buffer[..n]);

                    match message.trim() {
                        "stop" => {
                            tracing::info!("Stopping server...");
                            let _ = loop_shutdown_tx.send(()); //Send shutdown signal to father
                        }
                        "status" => {
                            stream.write_all(b"Server is running").await?;
                        }
                        cmd => {
                            println!("Received unknown command: {cmd}");
                        }
                    }
                    Ok::<_, anyhow::Error>(())
                });
            }
        } 
    }
    //Send shutdown signal to other threads
    let _ = tx_shutdown.send(());
    tracing::info!("IPC Stopping...");
    Ok(())
}
