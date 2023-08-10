use once_cell::sync::OnceCell;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use nero_util::error::{NeroError, NeroErrorKind, NeroResult};

use crate::app::App;
use crate::apps::cors::CORS;
use crate::apps::not_found::NotFound;
use crate::server::Server;
use crate::settings::Settings;

pub static DB: Surreal<Client> = Surreal::init();

pub struct Project {
    pub apps: Vec<App>,
    pub not_found: OnceCell<App>,
}

impl Project {
    pub async fn new(apps: Vec<App>) -> NeroResult<Self> {
        let mut _self = Self {
            apps,
            not_found: OnceCell::new(),
        };

        if Settings::db().connect && DB.health().await.is_err() {
            Self::connect_to_db().await?;
        }

        if Settings::cors().is_allow_cors {
            _self.apps.push(CORS::app()?)
        }

        Ok(_self)
    }

    pub async fn connect_to_db() -> NeroResult<()> {
        let err = |e| NeroError::new(NeroErrorKind::ConnectToDB, e);
        DB.connect::<Ws>(Settings::db().db_addr.clone())
            .await
            .map_err(err)?;

        DB.signin(Root {
            username: &Settings::db().db_user,
            password: &Settings::db().db_password,
        })
        .await
        .map_err(err)?;

        DB.use_ns(&Settings::db().db_db)
            .use_db(&Settings::db().db_ns)
            .await
            .map_err(err)?;

        Ok(())
    }

    pub fn add_apps(&mut self, mut apps: Vec<App>) {
        self.apps.append(&mut apps)
    }

    pub fn add_app(&mut self, app: App) {
        self.apps.push(app)
    }

    pub async fn run(self) -> NeroResult<()> {
        Server::setup(&Settings::server().addr)
            .await?
            .run(self)
            .await
    }

    pub fn set_not_found(self, app: App) -> Self {
        if self.not_found.set(app).is_err() {
            panic!("Failed set not found")
        }

        self
    }

    pub fn get_not_found(&self) -> &App {
        self.not_found.get_or_init(NotFound::app)
    }
}
