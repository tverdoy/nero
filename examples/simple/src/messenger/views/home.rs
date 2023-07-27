use async_trait::async_trait;
use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;
use serde::Serialize;

pub struct HomeView;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String
}

#[async_trait]
impl View for HomeView {
    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        let data = User { id: 34, name: "Dima".to_string() };
        Responder::json(Status::Ok, data).await
    }
}
