use std::{error::Error, fmt};

#[derive(Debug)]
pub enum LSErrorKind {
    InvalidArguments,
    PermissionDenied,
}

#[derive(Debug)]
pub struct LSError {
    pub kind: LSErrorKind,
    pub message: String,
}

impl Error for LSError {}

impl fmt::Display for LSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            LSErrorKind::PermissionDenied => write!(f, "permission denied: {}", self.message),
            LSErrorKind::InvalidArguments => write!(f, "invalid arguments: {}", self.message),
        }
    }
}
