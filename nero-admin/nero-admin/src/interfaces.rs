use serde::Serialize;

use crate::models::admin_user::AdminUser;

#[derive(Serialize)]
pub struct InterfaceLogin {
    pub token: String,
    pub user: AdminUser,
}
