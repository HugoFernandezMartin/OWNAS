use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::broadcast::Sender;

use crate::Commands;
use crate::core::responses::{DaemonResponse, ResponseType};
use crate::core::state::ServerStatus;
use crate::files::FilesCommands;
use crate::run::RunCommands;
use crate::{Server, file_manager::*};

pub async fn handle_ipc_connection(
    stream: UnixStream,
    loop_shutdown_tx: Sender<()>,
    server_clone: Arc<Server>,
    workspace_path: &str,
) -> anyhow::Result<()> {
    //Read command from CLI
    let (command, stream) = match receive_command(stream).await {
        Ok((c, s)) => (c, s),
        Err(e) => {
            tracing::error!(error = %e, "Unable to receive IPC command");
            anyhow::bail!("Unable to receive IPC command: {e}");
        }
    };

    tracing::info!("IPC Command received: {}", command);

    //Manage according to command
    match command {
        Commands::Ping => {
            //Just respose to CLI
            let response = DaemonResponse::Success(ResponseType::Info("Pong".to_string()));

            send_response(stream, response).await?;

            tracing::debug!("Ping response sent successfully");
        }
        Commands::Stop => {
            //Send shutdown signal to this and other threads
            let response =
                DaemonResponse::Success(ResponseType::Info("Stopping server...".to_string()));

            send_response(stream, response).await?;

            tracing::debug!("Shutdown signal received, stopping server...");

            //Send shutdown signal to father
            if let Err(e) = loop_shutdown_tx.send(()) {
                tracing::error!(error = %e, "Unable to send IPC loop shutdown signal to father");
                anyhow::bail!("Unable to send IPC loop shutdown signal to father: {e}");
            }
        }

        Commands::Status => {
            //Get status from server data and response to CLI
            let status: ServerStatus = server_clone.get_status();

            let response = DaemonResponse::Success(ResponseType::Status(status));

            send_response(stream, response).await?;

            tracing::debug!("Status response sent succesfully");
        }

        Commands::Run { subcommand } => match subcommand {
            RunCommands::ShowLog => {
                //Read log file and send to CLI
                let response = match server_clone.get_log().await {
                    Ok(log) => DaemonResponse::Success(ResponseType::Info(log)),
                    Err(e) => {
                        tracing::error!(error = %e, "Unable to get log");
                        DaemonResponse::Error(e.to_string())
                    }
                };

                send_response(stream, response).await?;

                tracing::debug!("Log response sent succesfully");
            }
        },
        Commands::Files { subcommand } => match subcommand {
            FilesCommands::List => {
                //Read workspace directory and send all file names in it
                let response = match list_files(workspace_path).await {
                    Ok(files) => DaemonResponse::Success(ResponseType::Files(files)),
                    Err(e) => {
                        tracing::error!(error = %e, "Unable to list files from directory");
                        DaemonResponse::Error(e.to_string())
                    }
                };

                send_response(stream, response).await?;

                tracing::debug!("List files response sent succesfully");
            }
            FilesCommands::Create { file_name } => {
                //Create a new empty file in workspace directory
                let response = match create_file(workspace_path, &file_name).await {
                    Ok(()) => DaemonResponse::Success(ResponseType::Info(
                        "File created succesfully".to_string(),
                    )),
                    Err(e) => {
                        tracing::error!(error = %e, "Unable to create file");
                        DaemonResponse::Error(e)
                    }
                };

                send_response(stream, response).await?;

                tracing::debug!("Create file response sent succesfully");
            }
            FilesCommands::Delete { file_name } => {
                //Delete a file from workspace directory given its name
                let response = match delete_file(workspace_path, &file_name).await {
                    Ok(()) => DaemonResponse::Success(ResponseType::Info(
                        "File deleted succesfully".to_string(),
                    )),
                    Err(e) => {
                        tracing::error!(error = %e, "Unable to delete file");
                        DaemonResponse::Error(e)
                    }
                };

                send_response(stream, response).await?;

                tracing::debug!("Delete file response sent succesfully");
            }
        },
        cmd => {
            //The other commands must never reach server cause CLI prevent that from happen
            tracing::error!("{cmd} command received: NOT SUPPOSED TO HAPPEN");
            anyhow::bail!("{cmd} command received: NOT SUPPOSED TO HAPPEN");
        }
    }
    tracing::info!("Command processed succesfully");
    Ok::<_, anyhow::Error>(())
}

async fn send_response(stream: UnixStream, response: DaemonResponse) -> anyhow::Result<()> {
    if let Err(e) = send_response_handler(stream, response).await {
        tracing::error!(error = %e, "Unable to send IPC response");
        anyhow::bail!("Unable to send IPC response: {e}");
    }
    Ok(())
}

async fn send_response_handler(
    mut stream: UnixStream,
    response: DaemonResponse,
) -> anyhow::Result<()> {
    tracing::debug!("Trying to send response");
    // Parse to JSON
    let json = serde_json::to_string(&response)
        .map_err(|e| anyhow::anyhow!("Failed to serialize response: {}", e))?;

    // Send to client
    stream.write_all(json.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

async fn receive_command(mut stream: UnixStream) -> Result<(Commands, UnixStream), anyhow::Error> {
    tracing::debug!("Trying to receive command");

    //Receive length
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf);

    let mut buffer = vec![0u8; len as usize];
    stream.read_exact(&mut buffer).await?;

    let command: Commands = serde_json::from_slice(&buffer)?;
    Ok((command, stream))
}
