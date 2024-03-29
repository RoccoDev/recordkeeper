use std::convert::Infallible;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("Data file is unknown or unsupported.")]
    UnrecognizedFormat,
    #[error(
        "Unsupported version {0}. Please update the game to the latest version, \
    then save again in the new version. (Only version {1} is supported.)"
    )]
    UnsupportedVersion(u32, u32),
    #[error("{0}")]
    AssertionError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unreachable")]
    Infallible(#[from] Infallible),
    #[error("Unexpected EOF while writing byte")]
    UnexpectedEof,
    #[error("Could not set crafted accessory data: the craft inventory is full.")]
    MashaInventoryFull,
}
