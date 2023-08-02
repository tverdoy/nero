use models::*;
use nero::app::App;
use nero::urlpatterns::UrlPatterns;
use views::*;

pub mod models;
pub mod views;

use nero::db::model::Scheme;

pub struct AdminView {
    apps: Vec<AdminAppView>,
}

pub struct AdminAppView {
    pub name: String,
    pub models: Vec<Scheme>,
}

pub async fn build_app(apps: &[App]) -> App {
    let mut patterns = UrlPatterns::default();
    patterns.add(vec![("/admin/login", Box::new(login::Login))]);
    patterns.add(vec![(
        "/admin/settings",
        Box::new(getsettings::GetSettings),
    )]);

    App::new(
        "Admin panel",
        patterns,
        vec![Box::<admin_user::AdminUser>::default()],
    )
}
