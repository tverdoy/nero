use crate::app::App;
use crate::server::Server;
use crate::urlpatterns::UrlPatterns;
use nero_util::error::NeroResult;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

pub struct Project {
    settings: Settings,
    apps: Vec<App>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            apps: Vec::new(),
        }
    }

    pub fn add_apps(&mut self, apps: Vec<App>) {
        for app in apps {
            self.apps.push(app)
        }
    }

    pub async fn run(&mut self) -> NeroResult<()> {
        let mut server = Server::setup(self.settings.addr).await?;
        let patters: Vec<UrlPatterns> = self.apps.iter().map(|app| app.url_patters()).collect();
        let all_patterns = UrlPatterns::merge_all(patters);

        server.run(all_patterns).await;

        Ok(())
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
