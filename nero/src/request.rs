use std::collections::HashMap;
use serde::de;
use serde::de::DeserializeOwned;
use nero_util::http::{ContentType, HeadReq};
use tokio::net::TcpStream;
use crate::error::{Error, ErrorKind};
use crate::error::*;

pub struct Request {
    pub socket: TcpStream,
    pub head: HeadReq,
    pub data: Option<Vec<u8>>
}

impl Request {
    pub fn new(socket: TcpStream, head: HeadReq, data: Option<Vec<u8>>) -> Self {
        Self { socket, head, data }
    }

    pub fn data_to_obj<'a, T: de::Deserialize<'a>>(&'a self) -> Result<T> {
        let data = self.data.as_ref().ok_or(Error::new_simple(ErrorKind::RequestDataIsNone))?;

        serde_json::from_slice(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }
}