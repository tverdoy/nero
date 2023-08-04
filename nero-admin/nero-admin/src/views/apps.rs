use async_trait::async_trait;

use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

use crate::interfaces::InterfaceApp;
use crate::models::admin_user::AdminUser;

pub struct GetAppsView {
    apps: Vec<InterfaceApp>,
}

impl GetAppsView {
    pub fn new(apps: Vec<InterfaceApp>) -> GetAppsView {
        Self { apps }
    }
}

#[async_trait]
impl View for GetAppsView {
    fn name(&self) -> &'static str {
        "get apps view"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;

        Responder::json(Status::Ok, &self.apps)
    }
}
