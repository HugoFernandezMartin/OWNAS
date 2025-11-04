pub mod run;
pub mod files;

use core::fmt;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

use crate::{commands::files::FilesCommands, run::RunCommands};

#[derive(Subcommand, Serialize, Deserialize)]
pub enum Commands {
    Start,
    Stop,
    Status,
    Run {
        #[command(subcommand)]
        subcommand: RunCommands,
    },
    Files {
        #[command(subcommand)]
        subcommand: FilesCommands,
    },
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "start")?,
            Self::Stop => write!(f, "stop")?,
            Self::Status => write!(f, "status")?,
            Self::Run { subcommand } => write!(f, "run {}", subcommand)?,
            Self::Files { subcommand } => write!(f, "files {}", subcommand)?,
        }
        Ok(())
    }
}

