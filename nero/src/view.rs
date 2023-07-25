use crate::error::*;
use crate::request::Request;
use async_trait::async_trait;
use crate::responder::Responder;

#[async_trait]
pub trait View {
    async fn callback(&self, request: &mut Request) -> Result<Responder>;
}
