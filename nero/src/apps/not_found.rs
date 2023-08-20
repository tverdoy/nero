use async_trait::async_trait;

use nero_util::http::Status;

use crate::request::Request;
use crate::responder::Responder;
use crate::view::View;


pub struct NotFoundView;

#[async_trait]
impl View for NotFoundView {
    fn name(&self) -> &'static str {
        "Not found"
    }

    async fn callback(&self, _request: &mut Request) -> crate::error::Result<Responder> {
        Responder::text(Status::NotFound, "Not found")
    }
}
