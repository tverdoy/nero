use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use nero_util::error::{NeroError, NeroErrorKind, NeroResult};

use crate::app::App;
use crate::apps::cors::CorsView;
use crate::apps::not_found::NotFoundView;
use crate::server::Server;
use crate::settings::Settings;
use tokio::sync::{Mutex, MutexGuard};
use crate::urlpatterns::Callback;

pub static DB: Surreal<Client> = Surreal::init();
pub static NOT_FOUND_VIEW: OnceCell<Callback> = OnceCell::new();
pub static CORS_VIEW: OnceCell<Callback> = OnceCell::new();


lazy_static! {
    pub static ref APPS: Mutex<Vec<App>> = Mutex::new(Vec::new());
}

pub struct Project;
impl Project {
    pub async fn run() -> NeroResult<()> {
        Server::setup(&Settings::server().addr)
            .await?
            .run()
            .await
    }

    pub async fn register_app(app: App) {
        APPS.lock().await.push(app);
    }

    pub async fn apps<'a>() -> MutexGuard<'a, Vec<App>> {
        APPS.lock().await
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

    pub fn set_not_found(callback: Callback) {
        if NOT_FOUND_VIEW.set(callback).is_err() {
            panic!("Failed set not found")
        }
    }

    pub fn not_found_view() -> &'static Callback {
        NOT_FOUND_VIEW.get_or_init(|| Box::new(NotFoundView))
    }

    pub fn set_cors_app(callback: Callback) {
        if CORS_VIEW.set(callback).is_err() {
            panic!("Failed set cors")
        }
    }

    pub fn cors_view() -> &'static Callback {
        CORS_VIEW.get_or_init(|| Box::new(CorsView))
    }
}

