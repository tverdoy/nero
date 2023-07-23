use nero_util::error::*;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use nero_util::http::HttpHeadReq;

pub const MAX_HTTP_HEADER_SIZE: usize = 4096;       // 4 KB
pub const MAX_HTTP_BODY_SIZE: usize = 4_194_304;    // 4 MB

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn setup<T: ToSocketAddrs>(addr: T) -> Result<Self> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| Error::new_simple(ErrorKind::SetupServer))?;

        Ok(Self { listener })
    }

    pub async fn run(&mut self) -> ! {
        loop {
            match self.listener.accept().await {
                Ok((socket, addr)) => match Self::handle_conn(socket).await {
                    Ok(_) => continue,
                    Err(e) => e.print(),
                },
                Err(e) => {
                    Error::new(ErrorKind::AcceptConnection, e).print();
                    continue;
                }
            };
        }
    }

    pub async fn handle_conn(mut socket: TcpStream) -> Result<()> {
        let head_bin = Self::read_req_head(&mut socket).await?;

        let head_string = String::from_utf8_lossy(&head_bin).to_string();
        println!("{head_string}");

        let head = HttpHeadReq::parse_from_utf8(&head_bin).unwrap();
        dbg!(head);

        Ok(())
    }


    pub async fn read_req_head(socket: &mut TcpStream) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut i = 0;

        while i < MAX_HTTP_HEADER_SIZE {
            let read_byte = socket
                .read_u8()
                .await
                .map_err(|e| Error::new_simple(ErrorKind::AcceptHttpHeader))?;
            buf.push(read_byte);

            if buf.len() > 3 && buf.ends_with(&[b'\r', b'\n', b'\r', b'\n']) {
                break;
            }

            i += 1;
        }

        Ok(buf)
    }
}
