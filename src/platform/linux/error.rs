use thiserror::Error;
use x11rb::errors::{ConnectError, ConnectionError, ReplyError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{}", .0)]
    Connect(#[from] ConnectError),
    #[error("{}", .0)]
    Connection(#[from] ConnectionError),
    #[error("{}", .0)]
    Reply(#[from] ReplyError),
}
