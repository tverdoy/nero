use tokio::net::TcpStream;
use nero_util::http::HttpHeadReq;

pub struct Request {
    socket: TcpStream,
    http_head: HttpHeadReq
}