use async_trait::async_trait;
use serde::Serialize;

use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::settings::{CorsConf, DataBaseConf, ServerConf};
use nero::settings::Settings;
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct ViewGetSettings;

#[derive(Serialize)]
struct RespDataGet {
    server: &'static ServerConf,
    db: &'static DataBaseConf,
    cors: &'static CorsConf,
}

#[async_trait]
impl View for ViewGetSettings {
    fn name(&self) -> &'static str {
        "Get settings"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;

        Responder::json(
            Status::Ok,
            RespDataGet {
                server: Settings::server(),
                db: Settings::db(),
                cors: Settings::cors(),
            },
        )
    }
}
