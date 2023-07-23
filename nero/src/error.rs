use std::fmt::{Debug, Formatter};
use std::{error, result};

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    error_type: ErrorType,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, err: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            error_type: ErrorType::Custom(kind, err.into()),
        }
    }

    pub fn new_simple(kind: ErrorKind) -> Self {
        Self {
            error_type: ErrorType::Simple(kind),
        }
    }

    pub fn print(&self) {
        eprintln!("{self:?}")
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("{kind:?}")),
            ErrorType::Custom(kind, err) => f.write_fmt(format_args!("{kind:?}: {err:?}")),
        }
    }
}

enum ErrorType {
    Simple(ErrorKind),
    Custom(ErrorKind, Box<dyn error::Error + Send + Sync>),
}

#[derive(Debug)]
pub enum ErrorKind {}
