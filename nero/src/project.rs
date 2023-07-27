use crate::app::App;
use crate::server::Server;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Connect, Connection, Response, Surreal};

pub struct Project {
    settings: Settings,
    apps: Vec<App>,
    db: Surreal<Client>,
}

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize)]
struct User<'a> {
    name: &'a str
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl Project {
    pub async fn new(settings: Settings, apps: Vec<App>) -> NeroResult<Self> {
        let db = Self::connect_to_db(&settings).await?;

        Ok(Self { settings, apps, db })
    }

    pub async fn connect_to_db(settings: &Settings) -> NeroResult<Surreal<Client>> {
        let err = |e| NeroError::new(NeroErrorKind::ConnectToDB, e);
        let db = Surreal::new::<Ws>(settings.db_addr.clone()).await.map_err(err)?;
        db.signin(Root {
            username: &settings.db_user,
            password: &settings.db_password,
        })
            .await
            .map_err(err)?;

        db.use_ns(&settings.db_db).use_db(&settings.db_ns).await.map_err(err)?;

        Ok(db)
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
    pub addr: String,

    pub db_addr: String,
    pub db_user: String,
    pub db_password: String,
    pub db_ns: String,
    pub db_db: String,

    pub max_head_size: usize,
    pub max_body_size: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:8080".to_string(),

            db_addr: "127.0.0.1:8000".to_string(),
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_ns: "nero".to_string(),
            db_db: "nero".to_string(),

            max_head_size: 4096,      // 4 KB
            max_body_size: 4_194_304, // 4 MB
        }
    }
}
