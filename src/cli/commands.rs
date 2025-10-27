use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Stop,
    Status
}