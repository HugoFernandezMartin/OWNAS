pub mod commands;
pub mod client;

use clap::{Parser};

use commands::Commands;

#[derive(Parser)]
#[command(name = "ownas")]
#[command(about = "OWNAS - Server Control from CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}