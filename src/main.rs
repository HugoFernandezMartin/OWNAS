mod server;
mod config;
mod core;
mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::loader::load_config("config/dev.json")?;
    logging::init::init_logging(&cfg.logging)?;
    let server = server::builder::ServerBuilder::new(cfg).build();
    let _ = server.start().await;
    Ok(())
}
