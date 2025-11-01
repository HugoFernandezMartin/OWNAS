use core::fmt;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::Config;


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    status: Status,
    pid: u32,
    uptime: Duration,
    log_file: String,
    log_level: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Running,
    NotAvailable
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "Running")?,
            Self::NotAvailable => write!(f, "Not Available, work in progress")?,
        }
        Ok(())
    }
}

impl ServerStatus {
    pub fn new(status: Status, pid: u32, start_time: Instant, cfg: &Config) -> ServerStatus {
        let uptime = Instant::now() - start_time;
        ServerStatus { status, pid, uptime, log_file: cfg.logging.logfile_path.clone(), log_level: cfg.logging.tracing_level.clone() }
    }
}

//Format for status
impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Server status: {}", self.status)?;
        writeln!(f, "PID: {}", self.pid)?;
        writeln!(f, "Uptime: {} seconds", self.uptime.as_secs())?;
        writeln!(f, "LogFile: {}", self.log_file)?;
        writeln!(f, "LogLevel: {}", self.log_level)?;
        Ok(())
    }
}