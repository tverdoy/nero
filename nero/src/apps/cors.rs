use async_trait::async_trait;

use crate::request::Request;
use crate::responder::Responder;
use crate::view::View;

pub struct CorsView;

#[async_trait]
impl View for CorsView {
    fn name(&self) -> &'static str {
        "Cors"
    }

    async fn callback(&self, _request: &mut Request) -> crate::error::Result<Responder> {
        Responder::no_content()
    }
}
