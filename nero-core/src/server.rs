use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use crate::error::*;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub const MAX_HTTP_HEADER_SIZE: usize = 4096;   // 4 kb

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

    pub async fn run(&mut self) -> ! { ;
        loop {
            match self.listener.accept().await {
                Ok((socket, addr)) => {
                    match Self::handle_conn(socket).await {
                        Ok(_) => continue,
                        Err(e) => e.print()
                    }
                },
                Err(e) => {
                    Error::new(ErrorKind::AcceptConnection, e).print();
                    continue
                },
            };


        }
    }

    pub async fn handle_conn(mut socket: TcpStream) -> Result<()> {
        Self::read_req_head(&mut socket).await?;

        Ok(())
    }

    pub async fn read_req_head(socket: &mut TcpStream) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut i = 0;

        while i < MAX_HTTP_HEADER_SIZE {
            let read_byte = socket.read_u8().await.map_err(|e| Error::new_simple(ErrorKind::AcceptHttpHeader))?;
            buf.push(read_byte);

            if buf.len() > 3 && buf.ends_with(&[b'\r', b'\n', b'\r', b'\n']) {
                break
            }

            i += 1;
        }

        Ok(buf)
    }
}
