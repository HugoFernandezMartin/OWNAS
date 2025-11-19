use std::{path::Path, sync::Arc};

use ownas::{
    builder,
    control::{ControlSignal, ErrorSeverity},
    init::init_logging,
    ipc_listener::run_ipc_listener,
    load_config,
    tcp_setup::run_tcp_server,
};

use tokio::sync::{broadcast, mpsc};

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

    'main: loop {
        //Load config.json
        let cfg = load_config("config/dev.json")?;

        if Path::new("/tmp/ownas.sock").exists() {
            eprintln!("Server already running");
            anyhow::bail!("Server already running");
        }

        //Build shared server struct
        let server = Arc::new(builder::ServerBuilder::new(cfg.clone()).build());

        tracing::info!("Server instance created...");

        //Create channel for main to shutdown threads
        let (shutdown_tx, _) = broadcast::channel::<()>(1);

        //Create channel for threads to talk with main
        let (control_tx, mut control_rx) = mpsc::channel::<ControlSignal>(10);

        tracing::debug!("Trying to start threads");

        //Throw server threads
        let ipc = tokio::spawn(run_ipc_listener(
            server.clone(),
            control_tx.clone(),
            shutdown_tx.subscribe(),
        ));

        let tcp = tokio::spawn(run_tcp_server(
            server.clone(),
            control_tx.clone(),
            shutdown_tx.subscribe(),
        ));

        tracing::debug!("Threads started");

        //Wait for threads to finish
        while let Some(signal) = control_rx.recv().await {
            match signal {
                ControlSignal::Restart => {
                    //Send shutdown signals to threads
                    if let Err(e) = shutdown_tx.send(()) {
                        tracing::error!(error = %e, "Unable to send shutdown signal to threads")
                    }

                    let _ = tokio::join!(ipc, tcp);

                    //Drop server instance
                    drop(server);

                    tracing::info!("Server restarted succesfully");

                    continue 'main;
                }
                ControlSignal::Shutdown => {
                    //Send shutdown signals to threads
                    if let Err(e) = shutdown_tx.send(()) {
                        tracing::error!(error = %e, "Unable to send shutdown signal to threads")
                    }

                    let _ = tokio::join!(ipc, tcp);

                    break 'main;
                }
                ControlSignal::Error {
                    source,
                    msg,
                    severity,
                } => match severity {
                    ErrorSeverity::Warning => {
                        tracing::warn!(error = %msg, source = {source}, "Critical error signal received from thread");
                        tracing::debug!("Trying to shutdown threads");
                        //Send shutdown signals to threads
                        if let Err(e) = shutdown_tx.send(()) {
                            tracing::error!(error = %e, "Unable to send shutdown signal to threads")
                        }
                        let _ = tokio::join!(ipc, tcp);
                        break 'main;
                    }
                    ErrorSeverity::Critical => {
                        tracing::error!(error = %msg, source = {source}, "Warning signal received from thread");
                        continue 'main;
                    }
                },
            }
        }
    }

    tracing::info!("Server shutting down...");

    std::process::exit(1)
}
