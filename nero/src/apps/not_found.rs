use crate::app::App;
use crate::request::Request;
use crate::responder::Responder;
use crate::urlpatterns::UrlPatterns;
use crate::view::View;
use async_trait::async_trait;
use nero_util::http::Status;

pub const NOT_FOUND_URL: &str = "/not-found";

pub struct NotFound;

impl NotFound {
    pub fn app() -> App {
        let mut patterns = UrlPatterns::default();
        patterns.add_one(NOT_FOUND_URL, Box::new(Self));

        App::new("not found", patterns, Vec::new())
    }
}

#[async_trait]
impl View for NotFound {
    fn name(&self) -> &'static str {
        "not found"
    }

    async fn callback(&self, _request: &mut Request) -> crate::error::Result<Responder> {
        Responder::text(Status::NotFound, "Not found")
    }
}
