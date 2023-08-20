use serde::de;
use tokio::net::TcpStream;

use nero_util::cookie::Cookie;
use nero_util::error::NeroResult;
use nero_util::http::{ContentType, HeadReq};

use crate::error::*;
use crate::error::{Error, ErrorKind};
use crate::server::Server;

pub struct Request {
    pub socket: TcpStream,
    pub head: HeadReq,
    pub body: Option<Vec<u8>>,
    pub set_cookie: Cookie,
}

impl Request {
    pub async fn init(mut socket: TcpStream, head: HeadReq) -> NeroResult<Self> {
        let mut body = None;
        if !head.is_acr() {
            if let Some(cont_len) = head.cont_len {
                body = Some(Server::read_req_body(&mut socket, cont_len).await?);
            }
        }

        Ok(Self {
            socket,
            head,
            body,
            set_cookie: Cookie::default(),
        })
    }

    pub fn clean_url(&self) -> String {
        self.head.url.rsplitn(2, '?').last().unwrap_or_default().to_string()
    }

    pub fn body_to_obj<'a, T: de::Deserialize<'a>>(&'a self) -> Result<T> {
        if self.head.cont_type != Some(ContentType::AppJson) {
            return Err(Error::new_simple(ErrorKind::RequestContentIsInvalid));
        };

        let body = self
            .body
            .as_ref()
            .ok_or(Error::new_simple(ErrorKind::RequestDataIsNone))?;

        serde_json::from_slice(body).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn params_to_obj<'a, T: de::Deserialize<'a>>(&'a self) -> Result<T> {
        if let Some(params) = self.head.url.splitn(2, '?').last() {
            serde_qs::from_str(params).map_err(|e| Error::new(ErrorKind::InvalidData, e))
        } else {
            Err(Error::new_simple(ErrorKind::RequestParamsIsNone))
        }
    }
}
