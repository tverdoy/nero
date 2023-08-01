use nero_util::error::{NeroError, NeroResult};
use std::fmt::{Debug, Display, Formatter};
use std::{error, result};
use serde::Serialize;
use nero_util::http::Status;
use crate::responder::Responder;

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
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("Error({kind:?})")),
            ErrorType::Custom(kind, err) => f.write_fmt(format_args!("Error({kind:?}): {err:?}")),
        }
    }
}

impl From<NeroError> for Error {
    fn from(value: NeroError) -> Self {
        Self::new(ErrorKind::Nero, value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::Simple(kind) => f.write_fmt(format_args!("Error ({kind:?})")),
            ErrorType::Custom(kind, err) => f.write_fmt(format_args!("Error({kind:?}) -> {err}")),
        }
    }
}

impl std::error::Error for Error {}

enum ErrorType {
    Simple(ErrorKind),
    Custom(ErrorKind, Box<dyn error::Error + Send + Sync>),
}

#[derive(Debug, Clone, Serialize)]
pub enum ErrorKind {
    Nero,
    InvalidData,
    Serialize,
    RequestDataIsNone,
    RequestContentIsInvalid,
    ObjectCreate,
    ObjectGet,
    ObjectDelete,
    ObjectUpdate,
    ObjectMerge,
    ObjectNotExists,
    ObjectIdIsNone,
    Auth
}

impl ErrorKind {
    pub fn to_status(&self) -> Status {
        match &self {
            Self::Nero | Self::ObjectCreate => Status::ServerError,
            Self::InvalidData | Self::Serialize | Self::RequestDataIsNone | Self::RequestContentIsInvalid | Self::ObjectIdIsNone => Status::BadRequest,
            Self::ObjectGet | Self::ObjectDelete | Self::ObjectUpdate | Self::ObjectMerge | Self::ObjectNotExists => Status::NotFound,
            Self::Auth => Status::Unauthorized
        }
    }
}

#[derive(Serialize)]
struct ErrorResp {
    kind: ErrorKind,
    error: String
}

impl Error {
    pub fn to_response(&self) -> Result<Responder> {
        let mut status = Status::ServerError;
        let mut text = String::new();

        match &self.error_type {
            ErrorType::Simple(kind) => {
                status = kind.to_status();
                text = status.status_info().1.to_owned()
            }
            ErrorType::Custom(kind, err) => {
                status = kind.to_status();
                text = format!("{err}")
            }
        }

        Responder::text(status, text)
    }

    pub fn to_json_response(&self) -> Result<Responder> {
        let mut status = Status::ServerError;
        let mut kind = ErrorKind::Nero;
        let mut text = String::new();

        match &self.error_type {
            ErrorType::Simple(_kind) => {
                status = _kind.to_status();
                text = status.status_info().1.to_owned();
                kind = _kind.clone();
            }
            ErrorType::Custom(_kind, err) => {
                status = _kind.to_status();
                text = format!("{err}");
                kind = _kind.clone();
            }
        }

        let err = ErrorResp { kind: kind, error: text };

        Responder::json(status, err)
    }
}