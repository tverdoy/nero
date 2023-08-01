use crate::error::*;
use crate::request::Request;
use crate::responder::Responder;
use async_trait::async_trait;
use nero_util::http::ContentType;

#[async_trait]
pub trait View {
    fn name(&self) -> &'static str;

    async fn callback(&self, request: &mut Request) -> Result<Responder>;

    async fn handler_error(&self, request: &mut Request, error: Error) -> Result<Responder> {
        if let Some(ContentType::AppJson) = request.head.cont_type {
            error.to_json_response()
        } else {
            error.to_json_response()
        }
    }
}
