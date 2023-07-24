use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use nero_util::http::{ContentType, HttpHeadReq, HttpHeadResp};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Request {
    socket: TcpStream,
    http_head: HttpHeadReq,
    is_closed: bool,
}

impl Request {
    pub fn new(socket: TcpStream, http_head: HttpHeadReq) -> Self {
        Self {
            socket,
            http_head,
            is_closed: false,
        }
    }

    pub async fn send(&mut self, header: HttpHeadResp, body: &[u8]) -> NeroResult<()> {
        let header = header.format_to_string();
        let data = [header.as_bytes(), body].concat();

        self.socket
            .write_all(&data)
            .await
            .map_err(|e| NeroError::new_simple(NeroErrorKind::SendResponse))?;

        Ok(())
    }

    pub fn http_head(&self) -> &HttpHeadReq {
        &self.http_head
    }

    pub async fn send_text<T: ToString>(&mut self, text: T) -> NeroResult<()> {
        let mut head = HttpHeadResp::default();
        let data = text.to_string();

        head.cont_type = ContentType::TextHtml;
        head.cont_len = data.len();

        self.send(head, data.as_bytes()).await
    }
}
