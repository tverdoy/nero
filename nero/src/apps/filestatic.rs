use crate::request::Request;
use crate::view::View;
use async_trait::async_trait;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use std::path::{Path, PathBuf};
use crate::responder::Responder;

pub struct FileStatic {
    root_url: String,
    root_fs: PathBuf,
}

impl FileStatic {
    pub fn new<T: ToString, P: AsRef<Path>>(root_url: T, root_fs: P) -> NeroResult<Self> {
        let root_fs = root_fs.as_ref().to_path_buf();
        if !root_fs.exists() {
            return Err(NeroError::new_simple(NeroErrorKind::FileNotFound));
        }

        Ok(Self {
            root_url: root_url.to_string(),
            root_fs,
        })
    }
}

#[async_trait]
impl View for FileStatic {
    async fn callback(&self, request: &mut Request) -> crate::error::Result<Responder> {
        todo!()
    }
}
