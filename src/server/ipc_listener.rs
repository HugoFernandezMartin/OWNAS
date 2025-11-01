use std::sync::Arc;

use tokio::fs;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::{net::UnixListener, sync::broadcast::{self,Sender}};

use crate::run::RunCommands;
use crate::{Commands, Server};
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
                tracing::trace!("IPC request received");
                let loop_shutdown_tx = loop_shutdown_tx.clone();
                let server_clone = server.clone();

                tokio::spawn(async move {
                    //Read command with length protocol
                    let mut len_buf = [0u8; 4];
                    stream.read_exact(&mut len_buf).await?;
                    let len = u32::from_be_bytes(len_buf);

                    let mut buffer = vec![0u8; len as usize];
                    stream.read_exact(&mut buffer).await?;

                    let command: Commands = serde_json::from_slice(&buffer)?;
                    tracing::trace!("Command received: {}", command);

                    match command {
                        Commands::Stop => {
                            let response = DaemonResponse::Info("Stopping server...".to_string());

                            send_response(stream, response).await?;

                            tracing::info!("Shutdown signal received, stopping server...");
                            let _ = loop_shutdown_tx.send(()); //Send shutdown signal to father
                        }
                        Commands::Status => {
                            let status: ServerStatus = server_clone.get_status();

                            let response = DaemonResponse::Status(status);

                            send_response(stream, response).await?;

                            tracing::trace!("Status response sended succesfully");
                        }
                        
                        Commands::Start => {
                            tracing::error!("Start command received: NOT SUPPOSED TO HAPPEN");
                        }

                        Commands::Run {subcommand} => {
                            match subcommand {
                                RunCommands::ShowLog => {
                                    let response = match server_clone.get_log() {
                                        Ok(log) => DaemonResponse::Info(log),
                                        Err(e) => DaemonResponse::Error(e.to_string()),
                                    };

                                    send_response(stream, response).await?;

                                    tracing::trace!("Log response sended succesfully");
                                }
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

async fn send_response(mut stream: UnixStream, response: DaemonResponse) -> anyhow::Result<()>{
    // Parse to JSON
    let json = serde_json::to_string(&response)
    .map_err(|e| anyhow::anyhow!("Failed to serialize response: {}", e))?;

    // Send to client
    stream.write_all(json.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}