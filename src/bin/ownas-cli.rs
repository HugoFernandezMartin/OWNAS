use ownas::{Cli, client::send_command, commands::Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
           let is_dev = std::env::var("OWNAS_DEV").is_ok();
           println!("{}", is_dev);
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
        }
        Commands::Stop => {
            send_command("stop").await?;
        }
        Commands::Status => {
            send_command("status").await?;
        }
    }

    Ok(())
}