use nero_util::http::HttpHeadReq;
use tokio::net::TcpStream;

pub struct Request {
    socket: TcpStream,
    http_head: HttpHeadReq,
}

impl Request {
    pub fn new(socket: TcpStream, http_head: HttpHeadReq) -> Self {
        Self { socket, http_head }
    }
}