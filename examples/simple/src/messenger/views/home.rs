use crate::messenger::models::user::User;
use async_trait::async_trait;
use nero::db::model::Object;
use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

pub struct HomeView;

#[async_trait]
impl View for HomeView {
    fn name(&self) -> &'static str {
        "Home"
    }

    async fn callback(&self, _request: &mut Request) -> nero::error::Result<Responder> {
        let user = User {
            id: Some("mavgu63j6x7ajnmpiafg".into()),
            name: "Masha".to_string(),
        };
        user.update().await?;

        Responder::text(Status::Ok, format!("{user:?}"))
    }
}
