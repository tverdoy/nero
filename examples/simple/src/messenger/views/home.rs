use async_trait::async_trait;
use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

pub struct HomeView;

#[async_trait]
impl View for HomeView {
    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        Responder::file(Status::Ok, "./src/messenger/templates/home.html").await
    }
}
