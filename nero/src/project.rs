use crate::app::App;
use crate::apps::cors::CORS;
use crate::apps::not_found::NotFound;
use crate::server::Server;
use crate::settings::{AuthTokenConf, CorsConf, DataBaseConf, ServerConf};
use nero_util::error::{NeroError, NeroErrorKind, NeroResult};
use once_cell::sync::OnceCell;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: Surreal<Client> = Surreal::init();
static SETTINGS: Settings = Settings::init();

pub struct Project {
    pub apps: Vec<App>,
    pub not_found: OnceCell<App>,
}

impl Project {
    pub async fn new(apps: Vec<App>) -> NeroResult<Self> {
        if DB.health().await.is_err() {
            Self::connect_to_db().await?;
        }

        Ok(Self {
            apps,
            not_found: OnceCell::new(),
        })
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

    pub async fn run(mut self) -> NeroResult<()> {
        if Settings::cors().is_allow_cors {
            self.apps.push(CORS::app()?)
        }

        for app in &self.apps {
            for model in app.models() {
                model.init().await;
            }
        }

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

pub struct Settings {
    pub server: OnceCell<ServerConf>,
    pub db: OnceCell<DataBaseConf>,
    pub cors: OnceCell<CorsConf>,
    pub admin_auth: OnceCell<AuthTokenConf>,
}

impl Settings {
    const fn init() -> Self {
        Self {
            server: OnceCell::new(),
            db: OnceCell::new(),
            cors: OnceCell::new(),
            admin_auth: OnceCell::new(),
        }
    }

    pub fn set_server(server: ServerConf) {
        SETTINGS.server.set(server).unwrap();
    }

    pub fn set_db(db: DataBaseConf) {
        SETTINGS.db.set(db).unwrap();
    }

    pub fn set_cors(cors: CorsConf) {
        SETTINGS.cors.set(cors).unwrap();
    }

    pub fn set_admin_auth(auth: AuthTokenConf) {
        SETTINGS.admin_auth.set(auth).unwrap();
    }

    pub fn server() -> &'static ServerConf {
        SETTINGS.server.get_or_init(ServerConf::default)
    }

    pub fn db() -> &'static DataBaseConf {
        SETTINGS.db.get_or_init(DataBaseConf::default)
    }

    pub fn cors() -> &'static CorsConf {
        SETTINGS.cors.get_or_init(CorsConf::default)
    }

    pub fn admin_auth() -> &'static AuthTokenConf {
        SETTINGS
            .admin_auth
            .get()
            .expect("Admin auth settings is not set")
    }
}
