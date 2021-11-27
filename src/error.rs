use std::{error::Error, fmt::Display, num::ParseIntError};
use tokio::io;

pub type SerirResult<T> = Result<T, SerirError>;

#[derive(Debug)]
pub enum SerirError {
    IoError(io::Error),
    ParseError(ParseIntError),
    RespParseError(String),
}

impl Display for SerirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerirError::IoError(e) => write!(f, "IO Error: {}", e),
            SerirError::ParseError(e) => write!(f, "Parsing error: {}", e),
            SerirError::RespParseError(msg) => write!(f, "Resp parsing error: {}", msg),
        }
    }
}

impl Error for SerirError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerirError::IoError(e) => Some(e),
            SerirError::ParseError(e) => Some(e),
            SerirError::RespParseError(_) => todo!(),
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
