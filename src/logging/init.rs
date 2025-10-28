use std::{fs, path::Path};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::FmtSubscriber;

use crate::config::LoggingConfig;

//Init logging tool
pub fn init_logging(logging_cfg: &LoggingConfig) -> Result<WorkerGuard, anyhow::Error> {
    let path = &logging_cfg.logfile_path;

    fs::write(path, "")?;
    
    let dir = Path::new(path).parent().unwrap();
    let file = Path::new(path).file_name().unwrap();

    // Crea un subscriber que escribe tanto en archivo como en consola
    let file_appender = tracing_appender::rolling::never(dir, file);
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(logging_cfg.get_tracing_level())
        .with_writer(file_writer)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(guard)
}