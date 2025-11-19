use std::{io::Error, thread, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::{Commands, core::responses::ServerResponse, handlers::ping_handler};

pub async fn receive_response(mut stream: UnixStream) -> Result<ServerResponse, Error> {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    let response: ServerResponse = serde_json::from_slice(&buf)?;
    Ok(response)
}

pub async fn send_command(mut stream: UnixStream, command: Commands) -> Result<UnixStream, Error> {
    let cmd_json = serde_json::to_vec(&command)?;
    let len = cmd_json.len() as u32;

    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(&cmd_json).await?;
    Ok(stream)
}

pub async fn wait_for_daemon(ipc_path: &str) -> bool {
    for _ in 0..30 {
        // 30 * 100ms = 3s total
        if let Some(stream) = test_connection(ipc_path).await {
            ping_handler(stream).await.unwrap();
            return true;
        };
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
