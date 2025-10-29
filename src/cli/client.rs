use std::{io::Error, thread, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};

use crate::core::responses::DaemonResponse;

pub async fn status_handler(mut stream: UnixStream, cmd: &str) -> anyhow::Result<()> {
    stream.write_all(cmd.as_bytes()).await?;

    match receive_response(stream).await? {
        DaemonResponse::Status(s) => println!("{}", s),
        DaemonResponse::Error(e) => eprintln!("Error executing status command: {}", e),
        _ => println!("Received unknown response"),
    }

    Ok(())
}

pub async fn stop_handler(mut stream: UnixStream, cmd: &str) -> anyhow::Result<()> {
    stream.write_all(cmd.as_bytes()).await?;

    match receive_response(stream).await? {
        DaemonResponse::Info(i) => println!("{}", i),
        DaemonResponse::Error(e) => eprintln!("Error executing stop command: {}", e),
        _ => println!("Received unknown response"),
    }

    Ok(())
}

pub async fn receive_response(mut stream: UnixStream) -> Result<DaemonResponse, Error> {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    let response: DaemonResponse = serde_json::from_slice(&buf)?;
    Ok(response)
}

pub async fn wait_for_daemon(ipc_path: &str) -> bool {
    for _ in 0..30 { // 30 * 100ms = 3s total
        if UnixStream::connect(ipc_path).await.is_ok() {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

pub async fn test_connection(ipc_path: &str) -> Option<UnixStream> {
    //Try to connect to ipc listener
    if let Ok(stream) = UnixStream::connect(ipc_path).await {
        Some(stream)
    } else {
        None
    }
}