use async_trait::async_trait;
use nero::app::App;
use serde::Serialize;

use nero::http::Status;
use nero::project::Project;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct GetAppsView;

#[derive(Serialize)]
struct ResponseGetApps<'a> {
    apps: &'a [App],
}

#[async_trait]
impl View for GetAppsView {
    fn name(&self) -> &'static str {
        "get apps view"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;
        let apps = Project::apps().await;

        Responder::json::<&[App]>(Status::Ok, apps.as_ref())
    }
}
