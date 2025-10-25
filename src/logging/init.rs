use tracing_subscriber::FmtSubscriber;

use crate::config::LoggingConfig;

//Init logging tool
pub fn init_logging(logging_cfg: &LoggingConfig) -> Result<(), String> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(logging_cfg.tracing_level)
        .finish();
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        return Err(e.to_string());
    }
    Ok(())
}