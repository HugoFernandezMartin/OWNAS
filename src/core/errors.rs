use std::io::Error;

#[derive(Debug)]
pub enum ServerError {
    IoError(Error),
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IoError(err)
    }
}