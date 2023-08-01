use crate::app::App;
use crate::apps::cors::CORS_URL;
use crate::apps::not_found::NOT_FOUND_URL;
use crate::project::Project;
use crate::request::Request;
use crate::responder::Responder;
use crate::urlpatterns::Callback;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::HeadReq;
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

    pub async fn run(&mut self, project: Project) -> ! {
        let project = Arc::new(project);

        loop {
            let project_view = project.clone();

            match self.listener.accept().await {
                Ok((socket, _addr)) => tokio::spawn(async move {
                    if let Err(e) = Self::handle_conn(socket, &project_view).await {
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

    pub async fn handle_conn(mut socket: TcpStream, project: &Project) -> NeroResult<()> {
        let head_bin = Self::read_req_head(&mut socket).await?;
        let head = HeadReq::parse_from_utf8(&head_bin).unwrap();
        let mut request = Request::init(socket, head).await?;

        let (app, view) = match Self::find_pattern(&project.apps, &request.head) {
            Some(pattern) => pattern,
            None => {
                eprintln!("Not found patter for: {}", request.head.url);
                return Self::not_found_patter(project, &mut request).await;
            }
        };

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
                    app.name(),
                    view.name(),
                    resp.head.status
                );
                resp
            }
        };

        responder.complete(&request);
        Self::send_response(&mut request.socket, &responder).await
    }

    pub async fn not_found_patter(project: &Project, request: &mut Request) -> NeroResult<()> {
        let app = project.get_not_found();
        let view = app
            .url_patters()
            .find_pattern(NOT_FOUND_URL)
            .unwrap()
            .clone();

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

    pub fn find_pattern<'a>(apps: &'a [App], head: &HeadReq) -> Option<(&'a App, Arc<Callback>)> {
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
