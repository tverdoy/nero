use serde::Serialize;

use nero::app::App;
use nero::db::scheme::Scheme;

use crate::models::admin_user::AdminUser;

#[derive(Serialize, Clone)]
pub struct InterfaceApp {
    pub name: String,
    pub schemes: Vec<Scheme>,
}

impl From<&App> for InterfaceApp {
    fn from(value: &App) -> Self {
        let mut schemes = Vec::new();

        for model in value.models() {
            schemes.push(model.scheme.clone())
        }

        Self {
            name: value.name().to_string(),
            schemes,
        }
    }
}

#[derive(Serialize)]
pub struct InterfaceLogin {
    pub token: String,
    pub user: AdminUser,
}
