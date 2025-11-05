use core::fmt;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Serialize, Deserialize)]
pub enum FilesCommands {
    List,
    Create { file_name: String },
    Delete { file_name: String },
    Write { file_name: String, text: String },
    Read { file_name: String },
}

impl fmt::Display for FilesCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List => write!(f, "list")?,
            Self::Create { file_name } => write!(f, "create {}", file_name)?,
            Self::Delete { file_name } => write!(f, "delete {}", file_name)?,
            Self::Write {
                file_name,
                text: _text,
            } => write!(f, "write {}", file_name)?,
            Self::Read { file_name } => write!(f, "read {}", file_name)?,
        }
        Ok(())
    }
}
