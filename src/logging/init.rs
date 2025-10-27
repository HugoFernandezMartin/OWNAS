use tracing_subscriber::FmtSubscriber;

use crate::config::LoggingConfig;

//Init logging tool
pub fn init_logging(logging_cfg: &LoggingConfig) -> Result<(), anyhow::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(logging_cfg.tracing_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}