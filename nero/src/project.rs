use crate::app::App;
use crate::server::Server;
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use crate::apps::cors::CORS;

pub static DB: Surreal<Client> = Surreal::init();

pub struct Project {
    settings: Settings,
    apps: Vec<App>,
}

impl Project {
    pub async fn new(settings: Settings, apps: Vec<App>) -> NeroResult<Self> {
        Self::connect_to_db(&settings).await?;

        Ok(Self { settings, apps })
    }

    pub async fn connect_to_db(settings: &Settings) -> NeroResult<()> {
        let err = |e| NeroError::new(NeroErrorKind::ConnectToDB, e);
        DB.connect::<Ws>(settings.db_addr.clone())
            .await
            .map_err(err)?;
        DB.signin(Root {
            username: &settings.db_user,
            password: &settings.db_password,
        })
        .await
        .map_err(err)?;

        DB.use_ns(&settings.db_db)
            .use_db(&settings.db_ns)
            .await
            .map_err(err)?;

        Ok(())
    }

    pub fn add_apps(&mut self, mut apps: Vec<App>) {
        self.apps.append(&mut apps)
    }

    pub async fn run(mut self) -> NeroResult<()> {
        if self.settings.is_allow_cors {
            self.apps.push(CORS::app()?)
        }

        Server::setup(&self.settings.addr)
            .await?
            .run(self.apps, self.settings)
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

    pub is_allow_cors: bool,
    pub allow_origin: String,
    pub allow_headers: Vec<String>,
    pub allow_methods: Vec<String>
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

            is_allow_cors: true,
            allow_origin: "*".to_string(),
            allow_headers: vec!["*".to_string()],
            allow_methods: vec!["GET".to_string(), "POST".to_string(), "OPTIONS".to_string()],
        }
    }
}
