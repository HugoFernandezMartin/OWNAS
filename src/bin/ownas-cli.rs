use anyhow::Ok;
use ownas::{Cli, client::{status_handler, stop_handler, test_connection, wait_for_daemon}, commands::Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ipc_path = "/tmp/ownas.sock";
    let cli = Cli::parse();

    let stream = test_connection(ipc_path).await;

    match cli.command {
        Commands::Start => {
            //First check if server is started already
            if stream.is_some() {
                eprintln!("Server is already running");
                return Ok(())
            }

            //If dev mode, execute the comand with cargo
            //If not, execute the normal command
           let is_dev = std::env::var("OWNAS_DEV").is_ok();
            if is_dev {
                std::process::Command::new("cargo")
                    .args(&["run", "--quiet", "--bin", "ownas-daemon"])
                    .spawn()
                    .expect("Failed to start daemon");
            } else {
                std::process::Command::new("ownas-daemon")
                    .spawn()
                    .expect("Failed to start daemon");
            }
            
            if wait_for_daemon("/tmp/ownas.sock").await {
                println!("Server started successfully");
            } else {
                eprintln!("Cannot start server, check log");
            }
        }
        Commands::Stop => {
            if stream.is_none() {
                eprintln!("Server is offline");
                return Ok(())
            }

            if let Err(_) = stop_handler(stream.unwrap(), "stop").await {
                println!("Cannot send stop signal to server");
            }
        }
        Commands::Status => {
            if stream.is_none() {
                eprintln!("Server is offline");
                return Ok(())
            }

            if let Err(_) = status_handler(stream.unwrap(), "status").await {
                eprintln!("Error handling status command")
            }

        }
    }

    Ok(())
}
