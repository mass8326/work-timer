use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not open your work timer project file")]
    InvalidProjectFile,
    #[error("Could not determine process path for pid {}", .0)]
    ProcessPathNotFound(u32),

    #[error("{}", .0)]
    Platform(#[from] crate::platform::Error),

    #[error("{}", .0)]
    Iced(#[from] iced::Error),
    #[error("{}", .0)]
    Io(#[from] io::Error),
    #[error("{}", .0)]
    DeserializeToml(#[from] toml::de::Error),
    #[error("{}", .0)]
    SerializeToml(#[from] toml::ser::Error),
}
