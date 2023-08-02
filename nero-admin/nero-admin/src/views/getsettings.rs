use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use nero::error::{Error, ErrorKind};
use nero::http::Status;
use nero::project::Settings;
use nero::request::Request;
use nero::responder::Responder;
use nero::settings::{CorsConf, DataBaseConf, ServerConf};
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct GetSettings;

#[derive(Serialize)]
struct RespData {
    server: &'static ServerConf,
    db: &'static DataBaseConf,
    cors: &'static CorsConf,
}

#[async_trait]
impl View for GetSettings {
    fn name(&self) -> &'static str {
        "get settings"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;

        Responder::json(
            Status::Ok,
            RespData {
                server: Settings::server(),
                db: Settings::db(),
                cors: Settings::cors(),
            },
        )
    }
}
