use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::{ContentType, HttpHeadReq, HttpHeadResp};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Request {
    pub socket: TcpStream,
    pub head: HttpHeadReq,
}

impl Request {
    pub fn new(socket: TcpStream, http_head: HttpHeadReq) -> Self {
        Self {
            socket,
            head: http_head
        }
    }
}
