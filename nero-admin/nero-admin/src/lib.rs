use models::*;
use nero::app::App;
use nero::urlpatterns::UrlPatterns;
use once_cell::sync::OnceCell;
use serde::Serialize;
use views::*;

pub mod models;
pub mod views;

use crate::models::admin_user::{AdminUser, ADMIN_USER_SCHEME};
use nero::db::model::{Model, Scheme};

static ADMIN_VIEW: OnceCell<AdminView> = OnceCell::new();

#[derive(Serialize)]
pub struct AdminView {
    apps: Vec<AdminAppView>,
}

impl AdminView {
    pub fn from_apps(apps: &[App]) -> Self {
        Self {
            apps: apps.iter().map(AdminAppView::from_app).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct AdminAppView {
    pub name: String,
    pub models: Vec<&'static Scheme>,
}

impl AdminAppView {
    pub fn from_app(app: &App) -> Self {
        let mut _self = Self {
            name: app.name().to_string(),
            models: Vec::new(),
        };
        for model in app.models() {
            _self.models.push(model.scheme)
        }

        _self
    }
}

pub async fn build_app(apps: &[App]) -> App {
    let admin_model = Model::new(
        Box::<AdminUser>::default(),
        ADMIN_USER_SCHEME,
    );

    let mut patterns = UrlPatterns::default();
    patterns.add(vec![
        ("/admin/login", Box::new(login::Login)),
        ("/admin/settings", Box::new(getsettings::GetSettings)),
        ("/admin/admin-view", Box::new(apps::GetAppsView))
    ]);

    // if let Err(e) = ADMIN_VIEW.set(AdminView::from_apps(vec![apps, admin_model])) {
    //     panic!("Failed set admin view")
    // }

    App::new(
        "Admin panel",
        patterns,
        vec![admin_model],
    )
}
