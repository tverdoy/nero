use async_trait::async_trait;
use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;
use serde::Deserialize;
use nero::error::{Error, ErrorKind};
use nero_util::http::ContentType;
use crate::models::admin_user::AdminUser;

pub struct Login;

#[derive(Deserialize)]
struct Data {
    username: String,
    password: String,
}

#[async_trait]
impl View for Login {
    fn name(&self) -> &'static str {
        "login"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        let data: Data = request.data_to_obj()?;
        let user = AdminUser::get_by_username(&data.username).await?;

        if !user.check_login(data.password).await? {
            return Err(Error::new(ErrorKind::Auth, "Invalid credentials"))
        }

        user.auth(request).await?;
        Responder::text(Status::Ok, format!("Hello {}", data.username))
    }
}
