pub mod client;
pub mod handlers;

use clap::{Parser};

use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "ownas")]
#[command(about = "OWNAS - Server Control from CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}