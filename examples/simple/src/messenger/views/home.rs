use async_trait::async_trait;
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
        Responder::text(Status::Ok, "HEllo")
    }
}
