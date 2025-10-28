use anyhow::Ok;
use ownas::{Cli, client::{send_command, test_connection, wait_for_daemon}, commands::Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ipc_path = "/tmp/ownas.sock";
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            //First check if server is started already
            if test_connection(ipc_path).await {
                println!("Server is already running");
                return Ok(())
            }

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
            if let Err(_) = send_command("stop").await {
                    println!("Server is offline");
            }
        }
        Commands::Status => {
            if let Err(_) = send_command("status").await {
                    println!("Server is offline");
            }
        }
    }

    Ok(())
}
