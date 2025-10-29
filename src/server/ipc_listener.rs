use std::sync::Arc;

use tokio::fs;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::UnixListener, sync::broadcast::{self,Sender}};

use crate::Server;
use crate::core::responses::DaemonResponse;
use crate::core::state::ServerStatus;

pub async fn run_ipc_listener(server: Arc<Server>, tx_shutdown: Sender<()>) -> anyhow::Result<()> {
    let socket_path = "/tmp/ownas.sock";
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
            Ok((mut stream, _)) = listener.accept() => {
                let loop_shutdown_tx = loop_shutdown_tx.clone();
                let server_clone = server.clone();

                tokio::spawn(async move {
                    let mut buffer = vec![0; 1024];
                    let n = stream.read(&mut buffer).await?;
                    let message = String::from_utf8_lossy(&buffer[..n]);

                    match message.trim() {
                        "stop" => {
                            let response = DaemonResponse::Info("Stopping server...".to_string());

                            // Parse to JSON
                            let json = serde_json::to_string(&response)
                                .map_err(|e| anyhow::anyhow!("Failed to serialize response: {}", e))?;

                            // Send to client
                            stream.write_all(json.as_bytes()).await?;
                            stream.flush().await?;

                            tracing::info!("Shutdown signal received, stopping server...");
                            let _ = loop_shutdown_tx.send(()); //Send shutdown signal to father
                        }
                        "status" => {
                            let status: ServerStatus = server_clone.get_status();

                            let response = DaemonResponse::Status(status);

                            // Parse to JSON
                            let json = serde_json::to_string(&response)
                                .map_err(|e| anyhow::anyhow!("Failed to serialize response: {}", e))?;

                            // Send to client
                            stream.write_all(json.as_bytes()).await?;
                            stream.flush().await?;
                        }
                        
                        cmd => {
                            if !cmd.trim().is_empty() { //TODO change this: result of connect in waiting for daemon
                                println!("Received unknown command: {cmd}");
                            }   
                        }
                    }
                    Ok::<_, anyhow::Error>(())
                });
            }
        } 
    }
    //Send shutdown signal to other threads
    let _ = tx_shutdown.send(());
    tracing::info!("IPC Stopped");
    Ok(())
}
