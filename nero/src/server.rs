use crate::app::App;
use crate::apps::cors::{CORS_URL};
use crate::request::Request;
use crate::responder::Responder;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::{HeadReq};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
            .map_err(|e| NeroError::new(NeroErrorKind::SetupServer, e))?;

        Ok(Self { listener })
    }

    pub async fn run(&mut self, apps: Vec<App>) -> ! {
        let apps = Arc::new(apps);

        loop {
            let apps_view = apps.clone();

            match self.listener.accept().await {
                Ok((socket, _addr)) => tokio::spawn(async move {
                    if let Err(e) = Self::handle_conn(socket, &apps_view).await {
                        eprintln!("{e}")
                    }
                }),
                Err(e) => {
                    let err = NeroError::new(NeroErrorKind::AcceptConnection, e);
                    eprintln!("{err}");
                    continue;
                }
            };
        }
    }

    pub async fn handle_conn(mut socket: TcpStream, apps: &Vec<App>) -> NeroResult<()> {
        let head_bin = Self::read_req_head(&mut socket).await?;
        let head = HeadReq::parse_from_utf8(&head_bin).unwrap();
        let mut pattern = None;

        let mut search_url = head.url.as_str();

        if head.is_acr() {
            search_url = CORS_URL
        }

        for app in apps {
            if let Some(patt) = app.url_patters().find_pattern(search_url) {
                pattern = Some((app, patt.clone()))
            }
        }

        let (app, view) = pattern.ok_or(NeroError::new(
            NeroErrorKind::PatternNotFound,
            format!("for url: {}", &head.url),
        ))?;

        let mut data = None;
        if !head.is_acr() {
            if let Some(cont_len) = head.cont_len {
                data = Some(Self::read_req_body(&mut socket, cont_len).await?);
            }
        }

        let mut request = Request::new(socket, head, data);
        let responder = view.callback(&mut request).await;
        match responder {
            Ok(mut responder) => {
                responder.complete(&request);

                Self::send_response(&mut request.socket, &responder).await
            }
            Err(e) => Err(NeroError::new(
                NeroErrorKind::ViewFailed,
                format!("{}::{} -> {}", app.name(), view.name(), e),
            )),
        }
    }

    pub async fn send_response(socket: &mut TcpStream, resp: &Responder) -> NeroResult<()> {
        socket
            .write_all(&resp.to_http_bytes())
            .await
            .map_err(|_| NeroError::new_simple(NeroErrorKind::SendResponse))
    }

    pub async fn read_req_head(socket: &mut TcpStream) -> NeroResult<Vec<u8>> {
        let mut buf = Vec::new();
        let mut i = 0;

        while i < MAX_HTTP_HEADER_SIZE {
            let read_byte = socket
                .read_u8()
                .await
                .map_err(|_| NeroError::new_simple(NeroErrorKind::AcceptHttpHeader))?;
            buf.push(read_byte);

            if buf.len() > 3 && buf.ends_with(&[b'\r', b'\n', b'\r', b'\n']) {
                break;
            }

            i += 1;
        }

        if i == MAX_HTTP_HEADER_SIZE {
            Err(NeroError::new_simple(NeroErrorKind::OverflowHttpHeader))
        } else {
            Ok(buf)
        }
    }

    pub async fn read_req_body(socket: &mut TcpStream, cont_len: usize) -> NeroResult<Vec<u8>> {
        if cont_len > MAX_HTTP_BODY_SIZE {
            return Err(NeroError::new_simple(NeroErrorKind::OverflowHttpBody));
        }

        let mut buf = vec![0; cont_len];
        socket
            .read_exact(&mut buf)
            .await
            .map_err(|_| NeroError::new_simple(NeroErrorKind::AcceptHttpBody))?;

        Ok(buf)
    }
}
