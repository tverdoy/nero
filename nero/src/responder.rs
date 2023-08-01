use crate::error::*;
use crate::project::{Settings};
use crate::request::Request;
use nero_util::encode::EncodeAlgo;
use nero_util::error::*;
use nero_util::http::{ContentType, HeadResp, Status};
use serde::Serialize;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const SIZE_ENCODE: usize = 2_097_152; // 2 MB

pub struct Responder {
    pub data: Vec<u8>,
    pub head: HeadResp,
}

impl Responder {
    pub fn complete(&mut self, request: &Request) {
        if self.data.len() > SIZE_ENCODE {
            if let Some(algorithms) = &request.head.accept_encode {
                if let Some(algo) = algorithms.iter().find(|algo| *algo == &EncodeAlgo::Deflate) {
                    self.data = algo.encode(&self.data);
                    self.head.cont_encode = Some(algo.clone())
                }
            }
        }

        if request.head.origin.is_some() && Settings::cors().is_allow_cors {
            self.head.aca_origin = Some(Settings::cors().allow_origin.clone());
            self.head.aca_methods = Some(Settings::cors().allow_methods.clone());
            self.head.aca_headers = Some(Settings::cors().allow_headers.clone());
        }

        if !request.set_cookie.is_empty() {
            self.head.set_cookie = Some(request.set_cookie.clone())
        }

        self.head.cont_len = self.data.len();
    }

    pub fn to_http_bytes(&self) -> Vec<u8> {
        let header = self.head.format_to_string();

        [header.as_bytes(), &self.data].concat()
    }

    pub fn ok() -> Result<Self> {
        let head = HeadResp {
            status: Status::Ok,
            ..Default::default()
        };

        Ok(Self {
            data: Vec::new(),
            head,
        })
    }

    pub fn no_content() -> Result<Self> {
        let head = HeadResp {
            status: Status::NoContent,
            ..Default::default()
        };

        Ok(Self {
            data: Vec::new(),
            head,
        })
    }

    pub fn text<T: ToString>(status: Status, data: T) -> Result<Self> {
        let head = HeadResp {
            cont_type: ContentType::TextPlain,
            status,
            ..Default::default()
        };

        Ok(Self {
            data: Vec::from(data.to_string()),
            head,
        })
    }

    pub async fn file<P: AsRef<Path>>(status: Status, path: P) -> Result<Self> {
        let err = |e| NeroError::new(NeroErrorKind::IO, e);

        let path = path.as_ref();

        if !path.exists() {
            return Err(NeroError::new_simple(NeroErrorKind::FileNotFound).into());
        }

        let mut file = File::open(&path).await.map_err(err)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.map_err(err)?;

        let head = HeadResp {
            cont_type: ContentType::from_file(path),
            status,
            ..Default::default()
        };

        Ok(Self { data: buf, head })
    }

    pub async fn json<T>(status: Status, data: T) -> Result<Self>
    where
        T: Serialize,
    {
        let head = HeadResp {
            cont_type: ContentType::AppJson,
            status,
            ..Default::default()
        };
        let json = serde_json::to_string(&data).map_err(|e| Error::new(ErrorKind::Serialize, e))?;

        Ok(Self {
            data: Vec::from(json),
            head,
        })
    }
}
