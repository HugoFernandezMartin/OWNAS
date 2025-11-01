use tokio::net::UnixStream;

use crate::{Commands, client::{receive_response, send_command}, core::responses::DaemonResponse, run::RunCommands};

pub async fn status_handler(stream: UnixStream) -> anyhow::Result<()> {
    let stream = send_command(stream, Commands::Status).await?;
    match receive_response(stream).await? {
        DaemonResponse::Status(s) => println!("{}", s),
        DaemonResponse::Error(e) => eprintln!("Error executing status command: {}", e),
        _ => println!("Received unknown response"),
    }

    Ok(())
}

pub async fn stop_handler(stream: UnixStream) -> anyhow::Result<()> {
    let stream = send_command(stream, Commands::Stop).await?;

    match receive_response(stream).await? {
        DaemonResponse::Info(i) => println!("{}", i),
        DaemonResponse::Error(e) => eprintln!("Error executing stop command: {}", e),
        _ => println!("Received unknown response"),
    }

    Ok(())
}

pub async fn show_log_handler(stream: UnixStream) -> anyhow::Result<()> {
    let stream = send_command(stream, Commands::Run { subcommand: RunCommands::ShowLog }).await?;

    todo!();

    Ok(())
}