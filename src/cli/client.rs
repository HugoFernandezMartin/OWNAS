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
