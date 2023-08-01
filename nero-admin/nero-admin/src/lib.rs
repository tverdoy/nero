pub mod models;
pub mod views;

use models::*;
use views::*;

use nero::app::App;
use nero::urlpatterns::UrlPatterns;

pub async fn build_app() -> App {
    let mut patterns = UrlPatterns::default();
    patterns.add(vec![("/admin/login", Box::new(login::Login))]);

    App::new(
        "Admin panel",
        patterns,
        vec![Box::<admin_user::AdminUser>::default()],
    )
}
