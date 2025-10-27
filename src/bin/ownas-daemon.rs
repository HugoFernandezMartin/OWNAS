use std::{path::Path, sync::Arc};

use ownas::{builder, init::init_logging, ipc_listener::run_ipc_listener, load_config, tcp_listener::run_tcp_listener};
use tokio::sync::broadcast;


// src/bin/ownas-daemon.rs
use tracing::{info, error};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //Load config.json
    let cfg = load_config("config/dev.json")?;
    init_logging(&cfg.logging)?;

    if Path::new(cfg.ipc_socket()).exists() {
        error!("Server already running");
        std::process::exit(1)
    }


    //Build shared server struct
    let server = Arc::new(builder::ServerBuilder::new(cfg).build());

    info!("Daemon starting...");

    //Create channel of comunication between threads
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let ipc_shutdown_tx = shutdown_tx.clone();
    let tcp_shutdown_tx = shutdown_tx.clone();

    //Throw server threads
    let ipc = tokio::spawn(run_ipc_listener(server.clone(), ipc_shutdown_tx.clone()));
    let tcp = tokio::spawn(run_tcp_listener(server.clone() , tcp_shutdown_tx.subscribe()));

    info!("Listeners started");

    //Wait for threads to finish
    let _ = tokio::join!(tcp, ipc);
    
    info!("Daemon shutting down...");
    
    std::process::exit(1)
}