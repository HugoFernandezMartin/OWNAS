use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub async fn list_files(path: &str) -> anyhow::Result<Vec<String>> {
    tracing::debug!("Trying to list files");
    let mut files = Vec::new();
    let mut entries = fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry.file_name();
        files.push(file_name.to_string_lossy().into_owned());
    }
    Ok(files)
}

pub async fn create_file(dir_path: &str, file_name: &str) -> Result<(), String> {
    tracing::debug!("Trying to create file");
    let path = Path::new(dir_path).join(file_name);
    if let Err(e) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .await
    {
        if e.kind() == ErrorKind::AlreadyExists {
            return Err("File already exists".to_string());
        } else {
            return Err(e.to_string());
        }
    }
    Ok(())
}

pub async fn delete_file(dir_path: &str, file_name: &str) -> Result<(), String> {
    tracing::debug!("Trying to delete file");
    let path = Path::new(dir_path).join(file_name);
    if let Err(e) = fs::remove_file(path).await {
        if e.kind() == ErrorKind::NotFound {
            return Err("File does not exists".to_string());
        } else {
            return Err(e.to_string());
        }
    }

    Ok(())
}

pub async fn write_in_file(dir_path: &str, file_name: &str, text: &str) -> Result<(), Error> {
    tracing::debug!("Trying to write in file");
    let path = Path::new(dir_path).join(file_name);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .await?;

    file.write_all(format!("{}\n", text).as_bytes()).await?;

    Ok(())
}

pub async fn read_file(dir_path: &str, file_name: &str) -> Result<String, anyhow::Error> {
    tracing::debug!("Trying to read file");
    let path = Path::new(dir_path).join(file_name);
    let mut file = OpenOptions::new().read(true).open(path).await?;
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).await?;
    let data = String::from_utf8(buf)?.trim().to_string();

    Ok(data)
}

pub async fn ensure_dir(path: &str) -> Result<(), Error> {
    tracing::debug!("Trying to ensure directory");
    if let Err(e) = fs::create_dir_all(path).await {
        if e.kind() == ErrorKind::AlreadyExists {
            return Ok(());
        }
        return Err(e);
    }
    Ok(())
}
