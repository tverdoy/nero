use crate::app::App;
use crate::error::{Error, ErrorKind};
use crate::request::Request;
use crate::responder::Responder;
use crate::urlpatterns::UrlPatterns;
use crate::view::View;
use async_trait::async_trait;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::Status;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileStatic {
    root_url: String,
    root_fs: PathBuf,
}

impl FileStatic {
    pub fn app<T: ToString, P: AsRef<Path>>(root_url: T, root_fs: P) -> NeroResult<App> {
        let root_url = root_url.to_string();
        let mut root_fs = root_fs.as_ref().to_path_buf();

        if root_fs.iter().last().unwrap().to_string_lossy() != "/" {
            root_fs = root_fs.join("")
        }

        if !root_fs.exists() {
            return Err(NeroError::new_simple(NeroErrorKind::FileNotFound));
        }

        let urls = Self::urls_by_path(&root_url, &root_fs);
        let mut patterns = UrlPatterns::default();

        for url in urls {
            let view = Self {
                root_url: root_url.clone(),
                root_fs: root_fs.clone(),
            };
            patterns.add_one(url, Box::new(view));
        }

        Ok(App::new("file static", patterns, Vec::new()))
    }

    pub fn urls_by_path<T: ToString, P: AsRef<Path>>(root_url: T, path: P) -> Vec<String> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        let mut res = Vec::new();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                let url_path = entry
                    .path()
                    .to_string_lossy()
                    .to_string()
                    .replace(&path_str, &root_url.to_string());
                res.push(url_path)
            }
        }

        res
    }
}

#[async_trait]
impl View for FileStatic {
    fn name(&self) -> &'static str {
        "File static"
    }

    async fn callback(&self, request: &mut Request) -> crate::error::Result<Responder> {
        let without_pref = request.head.url.replace(&self.root_url, "");
        let path = self
            .root_fs
            .join(without_pref)
            .canonicalize()
            .map_err(|_| Error::new_simple(ErrorKind::InvalidData))?;

        Responder::file(Status::Ok, path).await
    }
}
