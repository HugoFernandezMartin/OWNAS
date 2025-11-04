use std::{io::{Error, ErrorKind}, path::Path};

use tokio::fs::{self, OpenOptions};

pub async fn list_files(path: &str) -> anyhow::Result<Vec<String>>{
    let mut files = Vec::new();
    let mut entries = fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry.file_name();
        files.push(file_name.to_string_lossy().into_owned());
    }
    Ok(files)
}

pub async fn create_file(dir_path: &str, file_name: &str) -> Result<(), Error> {
    let path = Path::new(dir_path).join(file_name);
    let _file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .await?;
    Ok(())
} 

pub async fn delete_file(dir_path: &str, file_name: &str) -> Result<(), Error> {
    let path = Path::new(dir_path).join(file_name);
    fs::remove_file(path).await?;

    Ok(())
}

pub async fn ensure_dir(path: &str) -> Result<(), Error> {
    if let Err(e) = fs::create_dir_all(path).await {
        if e.kind() == ErrorKind::AlreadyExists {
            return Ok(())
        }
        return Err(e)
    }
    Ok(())
}