use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::HeadReq;

use crate::app::App;
use crate::project::Project;
use crate::request::Request;
use crate::responder::Responder;
use crate::settings::Settings;
use crate::urlpatterns::Callback;

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

    pub async fn run(&mut self) -> ! {
        loop {
            match self.listener.accept().await {
                Ok((socket, _addr)) => tokio::spawn(async move {
                    if let Err(e) = Self::handle_conn(socket).await {
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

    pub async fn handle_conn(mut socket: TcpStream) -> NeroResult<()> {
        let head_bin = Self::read_req_head(&mut socket).await?;
        let head = HeadReq::parse_from_utf8(&head_bin).unwrap();
        let mut request = Request::init(socket, head).await?;

        if request.head.is_acr() {
            return Self::send_cors(&mut request).await;
        }

        let apps = Project::apps().await;
        let (app, view) = match Self::find_pattern( apps.as_ref(), &request) {
            Some(pattern) => pattern,
            None => {
                eprintln!("Not found patter for: {}", request.head.url);
                return Self::send_not_found(&mut request).await;
            }
        };

        let app_name = app.name.clone();
        drop(apps);

        let mut responder = match view.callback(&mut request).await {
            Ok(resp) => {
                println!("{} -> {}", request.head.url, resp.head.status);
                resp
            }
            Err(e) => {
                let error_info = e.to_string();

                let resp = view
                    .handler_error(&mut request, e)
                    .await
                    .map_err(|e| NeroError::new(NeroErrorKind::HandleErrorFailed, e))?;
                eprintln!(
                    "{}::{} -> {error_info} :: {}",
                    app_name,
                    view.name(),
                    resp.head.status
                );
                resp
            }
        };

        responder.complete(&request);
        Self::send_response(&mut request.socket, &responder).await
    }

    pub async fn send_not_found(request: &mut Request) -> NeroResult<()> {
        let view = Project::not_found_view();

        let mut responder = match view.callback(request).await {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("View not found failed");
                return Err(NeroError::new(NeroErrorKind::ViewFailed, err));
            }
        };

        responder.complete(request);
        Self::send_response(&mut request.socket, &responder).await
    }

    pub async fn send_cors(request: &mut Request) -> NeroResult<()> {
        let view = Project::cors_view();

        let mut responder = match view.callback(request).await {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("View cors failed");
                return Err(NeroError::new(NeroErrorKind::ViewFailed, err));
            }
        };

        responder.complete(request);
        Self::send_response(&mut request.socket, &responder).await
    }

    pub fn find_pattern<'a>(apps: &'a [App], request: &Request) -> Option<(&'a App, Arc<Callback>)> {
        let mut pattern = None;

        for app in apps {
            if let Some(patt) = app.patterns.find_pattern(request.clean_url()) {
                pattern = Some((app, patt.clone()))
            }
        }

        pattern
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

        while i < Settings::server().max_head_size {
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

        if i == Settings::server().max_head_size {
            Err(NeroError::new_simple(NeroErrorKind::OverflowHttpHeader))
        } else {
            Ok(buf)
        }
    }

    pub async fn read_req_body(socket: &mut TcpStream, cont_len: usize) -> NeroResult<Vec<u8>> {
        if cont_len > Settings::server().max_body_size {
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
