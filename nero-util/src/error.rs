use std::error;
use std::fmt::{Debug, Display, Formatter};

pub type NeroResult<T> = Result<T, NeroError>;

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
}

impl Debug for NeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("NeroError({kind:?})")),
            ErrorType::Custom(kind, err) => {
                f.write_fmt(format_args!("NeroError({kind:?}): {err:?}"))
            }
        }
    }
}

impl Display for NeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("NeroError({kind:?})")),
            ErrorType::Custom(kind, err) => {
                f.write_fmt(format_args!("NeroError({kind:?}) -> {err}"))
            }
        }
    }
}

impl error::Error for NeroError {}

enum ErrorType {
    Simple(NeroErrorKind),
    Custom(NeroErrorKind, Box<dyn error::Error + Send + Sync>),
}

#[derive(Debug)]
pub enum NeroErrorKind {
    SetupServer,
    AcceptConnection,
    AcceptHttpHeader,
    AcceptHttpBody,
    OverflowHttpHeader,
    OverflowHttpBody,
    ParseHttpHeader,
    PatternNotFound,
    SendResponse,
    FileNotFound,
    RequestIsClosed,
    IO,
    ViewFailed,
    HandleErrorFailed,
    ConnectToDB,
    GenerateToken,
}
