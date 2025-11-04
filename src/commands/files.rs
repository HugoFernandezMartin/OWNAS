use core::fmt;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Serialize, Deserialize)]
pub enum FilesCommands {
    List,
    Create {
        #[arg(short, long)]
        file_name: String,
    },
    Delete {
        #[arg(short, long)]
        file_name: String,
    },
}

impl fmt::Display for FilesCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List => write!(f, "list")?,
            Self::Create {file_name} => write!(f, "create {}", file_name)?,
            Self::Delete {file_name}=> write!(f, "delete {}", file_name)?,
        }
        Ok(())
    }
}