use crate::{
    Commands,
    client::{receive_response, send_command},
    core::responses::{ResponseType, ServerResponse},
    files::FilesCommands,
    run::RunCommands,
};
use anyhow::Result;
use tokio::net::UnixStream;

// === Helpers ===

async fn handle_standard_response(stream: UnixStream, context: &str) -> Result<()> {
    match receive_response(stream).await? {
        ServerResponse::Success(ResponseType::Info(msg)) => println!("{msg}"),
        ServerResponse::Error(err) => eprintln!("Error executing {context} command: {err}"),
        _ => eprintln!("Unexpected response for {context}"),
    }
    Ok(())
}

async fn handle_status_response(stream: UnixStream, context: &str) -> Result<()> {
    match receive_response(stream).await? {
        ServerResponse::Success(ResponseType::Status(status)) => print!("{status}"),
        ServerResponse::Error(err) => eprintln!("Error executing {context} command: {err}"),
        _ => eprintln!("Unexpected response for {context}"),
    }
    Ok(())
}

async fn handle_file_list_response(stream: UnixStream, context: &str) -> Result<()> {
    match receive_response(stream).await? {
        ServerResponse::Success(ResponseType::Files(files)) => {
            for file in files {
                println!("- {}", file);
            }
        }
        ServerResponse::Error(err) => eprintln!("Error executing {context} command: {err}"),
        _ => eprintln!("Unexpected response for {context}"),
    }
    Ok(())
}

// === Handlers ===

pub async fn ping_handler(stream: UnixStream) -> Result<()> {
    let stream = send_command(stream, Commands::Ping).await?;
    match receive_response(stream).await? {
        ServerResponse::Success(ResponseType::Info(_)) => {}
        ServerResponse::Error(err) => eprintln!("Error executing ping command: {err}"),
        _ => eprintln!("Unexpected response for ping"),
    }
    Ok(())
}

pub async fn status_handler(stream: UnixStream) -> Result<()> {
    let stream = send_command(stream, Commands::Status).await?;
    handle_status_response(stream, "status").await
}

pub async fn restart_handler(stream: UnixStream) -> Result<()> {
    let stream = send_command(stream, Commands::Restart).await?;
    handle_standard_response(stream, "restart").await
}

pub async fn stop_handler(stream: UnixStream) -> Result<()> {
    let stream = send_command(stream, Commands::Stop).await?;
    handle_standard_response(stream, "stop").await
}

pub async fn show_log_handler(stream: UnixStream) -> Result<()> {
    let stream = send_command(
        stream,
        Commands::Run {
            subcommand: RunCommands::ShowLog,
        },
    )
    .await?;
    handle_standard_response(stream, "show log").await
}

pub async fn list_files_handler(stream: UnixStream) -> Result<()> {
    let stream = send_command(
        stream,
        Commands::Files {
            subcommand: FilesCommands::List,
        },
    )
    .await?;
    handle_file_list_response(stream, "list files").await
}

pub async fn create_file_handler(stream: UnixStream, file_name: String) -> Result<()> {
    let stream = send_command(
        stream,
        Commands::Files {
            subcommand: FilesCommands::Create { file_name },
        },
    )
    .await?;
    handle_standard_response(stream, "create file").await
}

pub async fn delete_file_handler(stream: UnixStream, file_name: String) -> Result<()> {
    let stream = send_command(
        stream,
        Commands::Files {
            subcommand: FilesCommands::Delete { file_name },
        },
    )
    .await?;
    handle_standard_response(stream, "delete file").await
}

pub async fn write_file_handler(stream: UnixStream, file_name: String, text: String) -> Result<()> {
    let stream = send_command(
        stream,
        Commands::Files {
            subcommand: FilesCommands::Write { file_name, text },
        },
    )
    .await?;
    handle_standard_response(stream, "write file").await
}

pub async fn read_file_handler(stream: UnixStream, file_name: String) -> Result<()> {
    let stream = send_command(
        stream,
        Commands::Files {
            subcommand: FilesCommands::Read { file_name },
        },
    )
    .await?;
    handle_standard_response(stream, "read file").await
}
