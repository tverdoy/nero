use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::{ContentType, HttpHeadResp, Status};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::error::*;

pub struct Responder {
    pub data: Vec<u8>,
    pub head: HttpHeadResp,
}

impl Responder {
    pub fn complete(&mut self) {
        self.head.cont_len = self.data.len();
    }

    pub fn to_http_bytes(&self) -> Vec<u8> {
        let header = self.head.format_to_string();
        [header.as_bytes(), &self.data].concat()
    }

    pub fn ok() -> Result<Self> {
        let mut head = HttpHeadResp::default();
        head.status = Status::Ok;

        Ok(Self { data: Vec::new(), head })
    }

    pub fn text<T: ToString>(status: Status, data: T) -> Result<Self> {
        let mut head = HttpHeadResp::default();
        head.cont_type = ContentType::TextHtml;
        head.status = status;

        Ok(Self {
            data: Vec::from(data.to_string()),
            head,
        })
    }

    pub async fn file<P: AsRef<Path>>(status: Status, path: P) -> Result<Self> {
        let err = |e| NeroError::new(NeroErrorKind::IO, e);

        let mut head = HttpHeadResp::default();
        let path = path.as_ref();

        if !path.exists() {
            return Err(NeroError::new_simple(NeroErrorKind::FileNotFound).into());
        }

        let mut file = File::open(&path).await.map_err(err)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.map_err(err)?;

        head.cont_type = ContentType::from_file(path);
        head.status = status;

        Ok(Self {
            data: buf,
            head,
        })
    }
}
