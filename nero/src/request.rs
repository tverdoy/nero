use nero_util::http::HeadReq;
use tokio::net::TcpStream;

pub struct Request {
    pub socket: TcpStream,
    pub head: HeadReq,
}

impl Request {
    pub fn new(socket: TcpStream, head: HeadReq) -> Self {
        Self { socket, head }
    }
}
