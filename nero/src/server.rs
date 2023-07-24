use crate::error::{Error, ErrorKind, Result};
use crate::request::Request;
use crate::urlpatterns::UrlPatterns;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::HttpHeadReq;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub const MAX_HTTP_HEADER_SIZE: usize = 4096; // 4 KB
pub const MAX_HTTP_BODY_SIZE: usize = 4_194_304; // 4 MB

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn setup<T: ToSocketAddrs>(addr: T) -> NeroResult<Self> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| NeroError::new_simple(NeroErrorKind::SetupServer))?;

        Ok(Self { listener })
    }

    pub async fn run(&mut self, patterns: UrlPatterns) -> ! {
        let patterns = Arc::new(patterns);
        loop {
            let mut patterns_view = patterns.clone();
            match self.listener.accept().await {
                Ok((socket, addr)) => tokio::spawn(async move {
                    if let Err(e) = Self::handle_conn(socket, &patterns_view).await {
                        e.print();
                    }
                }),
                Err(e) => {
                    NeroError::new(NeroErrorKind::AcceptConnection, e).print();
                    continue;
                }
            };
        }
    }

    pub async fn handle_conn(mut socket: TcpStream, patterns: &UrlPatterns) -> NeroResult<()> {
        let head_bin = Self::read_req_head(&mut socket).await?;

        let head_string = String::from_utf8_lossy(&head_bin).to_string();
        println!("{head_string}");

        let head = HttpHeadReq::parse_from_utf8(&head_bin).unwrap();
        match patterns.find_pattern(&head.url) {
            Some(view) => {
                let mut request = Request::new(socket, head);
                view.callback(&mut request).await.unwrap();
            }
            None => {
                return Err(NeroError::new(
                    NeroErrorKind::PatternNotFound,
                    format!("for url: {}", &head.url),
                ))
            }
        }

        Ok(())
    }

    pub async fn read_req_head(socket: &mut TcpStream) -> NeroResult<Vec<u8>> {
        let mut buf = Vec::new();
        let mut i = 0;

        while i < MAX_HTTP_HEADER_SIZE {
            let read_byte = socket
                .read_u8()
                .await
                .map_err(|e| NeroError::new_simple(NeroErrorKind::AcceptHttpHeader))?;
            buf.push(read_byte);

            if buf.len() > 3 && buf.ends_with(&[b'\r', b'\n', b'\r', b'\n']) {
                break;
            }

            i += 1;
        }

        Ok(buf)
    }
}
