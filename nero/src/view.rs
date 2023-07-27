use crate::error::*;
use crate::request::Request;
use crate::responder::Responder;
use async_trait::async_trait;

#[async_trait]
pub trait View {
    fn name(&self) -> &'static str;

    async fn callback(&self, request: &mut Request) -> Result<Responder>;
}
