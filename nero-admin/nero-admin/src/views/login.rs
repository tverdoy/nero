use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use nero::error::{Error, ErrorKind};
use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct Login;

#[derive(Deserialize)]
struct Data {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RespData {
    token: String,
    user: AdminUser
}

#[async_trait]
impl View for Login {
    fn name(&self) -> &'static str {
        "login"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        let err = || Error::new(ErrorKind::Auth, "Invalid credentials");
        let data: Data = request.data_to_obj()?;
        let mut user = AdminUser::get_by_username(&data.username).await.map_err(|_| err())?;

        if !user.check_login(data.password).await? {
            return Err(err());
        }

        let token = user.auth().await?;
        user.password = None;
        Responder::json(Status::Ok, RespData { token, user })
    }
}
