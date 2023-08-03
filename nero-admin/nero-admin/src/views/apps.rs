use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::ADMIN_VIEW;
use nero::error::{Error, ErrorKind};
use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct GetAppsView;

#[derive(Serialize)]
struct RespData {}

#[async_trait]
impl View for GetAppsView {
    fn name(&self) -> &'static str {
        "get apps view"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;
        let data = ADMIN_VIEW
            .get()
            .ok_or(Error::new(ErrorKind::Other, "Admin view not set"))?;

        Responder::json(Status::Ok, data)
    }
}
