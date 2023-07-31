use async_trait::async_trait;
use serde::Deserialize;
use nero::error::{Error, ErrorKind};
use nero::http::Status;
use nero::request::{Request};
use nero::responder::Responder;
use nero::view::View;

pub struct Login;

#[derive(Deserialize)]
struct Data {
    username: String,
    password: String
}

#[async_trait]
impl View for Login {
    fn name(&self) -> &'static str {
        "login"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        let err = || Error::new_simple(ErrorKind::InvalidData);

        let data: Data = request.data_to_obj()?;
        Responder::text(Status::Ok, format!("Hello {}", data.username))
    }
}