use core::fmt;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Serialize, Deserialize)]
pub enum RunCommands {
    ShowLog
}

impl fmt::Display for RunCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShowLog => write!(f, "show-log")?,
        }
        Ok(())
    }
}