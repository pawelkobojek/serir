use std::{error::Error, fmt::Display, num::ParseIntError};
use tokio::io;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

use crate::server::Request;

pub type SerirResult<T> = Result<T, SerirError>;

#[derive(Debug)]
pub enum SerirError {
    IoError(io::Error),
    ParseError(ParseIntError),
    RespParseError(String),
    MpscSendError(SendError<Request>),
    OneshotRecvError(RecvError),
}

impl Display for SerirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerirError::IoError(e) => write!(f, "IO Error: {}", e),
            SerirError::ParseError(e) => write!(f, "Parsing error: {}", e),
            SerirError::RespParseError(msg) => write!(f, "Resp parsing error: {}", msg),
            SerirError::MpscSendError(e) => write!(f, "Mpsc send error: {}", e),
            SerirError::OneshotRecvError(e) => write!(f, "Oneshot recv error: {}", e),
        }
    }
}

impl Error for SerirError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerirError::IoError(e) => Some(e),
            SerirError::ParseError(e) => Some(e),
            SerirError::MpscSendError(e) => Some(e),
            SerirError::OneshotRecvError(e) => Some(e),
            SerirError::RespParseError(_) => None,
        }
    }
}

impl From<io::Error> for SerirError {
    fn from(err: io::Error) -> Self {
        SerirError::IoError(err)
    }
}

impl From<ParseIntError> for SerirError {
    fn from(err: ParseIntError) -> Self {
        SerirError::ParseError(err)
    }
}

impl From<SendError<Request>> for SerirError {
    fn from(err: SendError<Request>) -> Self {
        SerirError::MpscSendError(err)
    }
}

impl From<RecvError> for SerirError {
    fn from(err: RecvError) -> Self {
        SerirError::OneshotRecvError(err)
    }
}
