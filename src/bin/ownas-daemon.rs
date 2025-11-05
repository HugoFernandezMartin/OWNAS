use std::{path::Path, sync::Arc};

use ownas::{
    builder, init::init_logging, ipc_listener::run_ipc_listener, load_config,
    tcp_listener::run_tcp_listener,
};
use tokio::sync::broadcast;

// src/bin/ownas-daemon.rs

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //Load config.json
    let cfg = load_config("config/dev.json")?;

    if Path::new("/tmp/ownas.sock").exists() {
        eprintln!("Server already running");
        anyhow::bail!("Server already running");
    }

    let _guard = init_logging(&cfg.logging)?;

    //Build shared server struct
    let server = Arc::new(builder::ServerBuilder::new(cfg).build());

    tracing::info!("Server starting...");

    //Create channel of comunication between threads
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let ipc_shutdown_tx = shutdown_tx.clone();
    let tcp_shutdown_tx = shutdown_tx.clone();

    tracing::debug!("Trying to start threads");

    //Throw server threads
    let ipc = tokio::spawn(run_ipc_listener(server.clone(), ipc_shutdown_tx.clone()));
    let tcp = tokio::spawn(run_tcp_listener(
        server.clone(),
        tcp_shutdown_tx.subscribe(),
    ));

    tracing::debug!("Threads started");

    //Wait for threads to finish
    let _ = tokio::join!(tcp, ipc);

    tracing::info!("Server shutting down...");

    std::process::exit(1)
}
