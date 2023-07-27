use crate::app::App;
use crate::server::Server;
use nero_util::error::NeroResult;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Project {
    settings: Settings,
    apps: Vec<App>,
}

impl Project {
    pub fn new(settings: Settings, apps: Vec<App>) -> Self {
        Self { settings, apps }
    }

    pub fn add_apps(&mut self, mut apps: Vec<App>) {
        self.apps.append(&mut apps)
    }

    pub async fn run(self) -> NeroResult<()> {
        Server::setup(self.settings.addr)
            .await?
            .run(self.apps)
            .await
    }
}

#[derive(Debug)]
pub struct Settings {
    pub addr: SocketAddr,
    pub max_head_size: usize,
    pub max_body_size: usize,
}

impl Settings {
    pub async fn set_addr(&mut self, addr: SocketAddr) {
        self.addr = addr
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            max_head_size: 4096,      // 4 KB
            max_body_size: 4_194_304, // 4 MB
        }
    }
}
