use std::fmt::{Debug, Display, Formatter};
use std::{error, result};

pub type NeroResult<T> = result::Result<T, NeroError>;

pub struct NeroError {
    error_type: ErrorType,
}

impl NeroError {
    pub fn new<E>(kind: NeroErrorKind, err: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            error_type: ErrorType::Custom(kind, err.into()),
        }
    }

    pub fn new_simple(kind: NeroErrorKind) -> Self {
        Self {
            error_type: ErrorType::Simple(kind),
        }
    }

    pub fn print(&self) {
        eprintln!("{self:?}")
    }
}

impl Debug for NeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("{kind:?}")),
            ErrorType::Custom(kind, err) => f.write_fmt(format_args!("{kind:?}: {err:?}")),
        }
    }
}

enum ErrorType {
    Simple(NeroErrorKind),
    Custom(NeroErrorKind, Box<dyn error::Error + Send + Sync>),
}

#[derive(Debug)]
pub enum NeroErrorKind {
    SetupServer,
    AcceptConnection,
    AcceptHttpHeader,
    ParseHttpHeader,
    PatternNotFound,
    SendResponse,
    FileNotFound,
    RequestIsClosed,
    IO,
    ViewFailed,
}

impl Display for NeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("NeroError ({kind:?})")),
            ErrorType::Custom(kind, err) => {
                f.write_fmt(format_args!("NeroError ({kind:?}): {err}"))
            }
        }
    }
}

impl std::error::Error for NeroError {}
