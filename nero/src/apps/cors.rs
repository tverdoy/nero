use crate::app::App;
use crate::request::Request;
use crate::responder::Responder;
use crate::urlpatterns::UrlPatterns;
use crate::view::View;
use async_trait::async_trait;
use nero_util::error::NeroResult;

pub struct CORS;

pub const CORS_URL: &str = "/cors/allow";

impl CORS {
    pub fn app() -> NeroResult<App> {
        let mut patterns = UrlPatterns::default();
        patterns.add_one(CORS_URL, Box::new(Self));

        Ok(App::new("cors", patterns, Vec::new()))
    }
}

#[async_trait]
impl View for CORS {
    fn name(&self) -> &'static str {
        "cors"
    }

    async fn callback(&self, _request: &mut Request) -> crate::error::Result<Responder> {
        Responder::no_content()
    }
}
