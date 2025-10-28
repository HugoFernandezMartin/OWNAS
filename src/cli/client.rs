use std::{thread, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};

pub async fn send_command(cmd: &str) -> anyhow::Result<()> {
    //Connect to the server through unix stream
    let mut stream = UnixStream::connect("/tmp/ownas.sock").await?;
    
    stream.write_all(cmd.as_bytes()).await?;

    let mut response = vec![0; 1024];
    let n = stream.read(&mut response).await?;
    println!("{}", String::from_utf8_lossy(&response[..n]));

    Ok(())
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

pub async fn test_connection(ipc_path: &str) -> bool {
    //Try to connect to ipc listener
    if let Ok(_) = UnixStream::connect(ipc_path).await {
        true
    } else {
        false
    }
}