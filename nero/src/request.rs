use crate::error::*;
use crate::error::{Error, ErrorKind};
use crate::server::Server;
use nero_util::cookie::Cookie;
use nero_util::error::NeroResult;
use nero_util::http::{ContentType, HeadReq};
use serde::de;
use tokio::net::TcpStream;

pub struct Request {
    pub socket: TcpStream,
    pub head: HeadReq,
    pub data: Option<Vec<u8>>,
    pub set_cookie: Cookie,
}

impl Request {
    pub async fn init(mut socket: TcpStream, head: HeadReq) -> NeroResult<Self> {
        let mut data = None;
        if !head.is_acr() {
            if let Some(cont_len) = head.cont_len {
                data = Some(Server::read_req_body(&mut socket, cont_len).await?);
            }
        }

        Ok(Self {
            socket,
            head,
            data,
            set_cookie: Cookie::default(),
        })
    }

    pub fn data_to_obj<'a, T: de::Deserialize<'a>>(&'a self) -> Result<T> {
        if self.head.cont_type != Some(ContentType::AppJson) {
            return Err(Error::new_simple(ErrorKind::RequestContentIsInvalid));
        };

        let data = self
            .data
            .as_ref()
            .ok_or(Error::new_simple(ErrorKind::RequestDataIsNone))?;

        serde_json::from_slice(data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }
}
