use codec::Error as CodecError;
use serde_json::Error as JsonError;
use std::{boxed::Box, sync::mpsc::RecvError};
use thiserror;
use ws::Error as WsClientError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Codec(#[from] CodecError),
    #[error("{0}")]
    SerdeJson(#[from] JsonError),
    #[error("Validateer returned the following error message: {0}")]
    Status(String),
    #[error("Websocket error: {0}")]
    WsClientError(#[from] WsClientError),
    #[error("Faulty channel: {0}")]
    MspcReceiver(#[from] RecvError),
    #[error("Custom Error: {0}")]
    Custom(Box<dyn std::error::Error + Sync + Send + 'static>),
}
