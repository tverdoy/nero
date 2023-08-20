use nero::app::App;
use nero::db::model::Model;
use nero::urlpatterns::UrlPatterns;

use crate::models::admin_user::AdminUser;
use crate::views::settings::GetSettingsView;
use nero::db::model::Manager;
use nero::project::Project;
use crate::views::apps::GetAppsView;
use crate::views::login::LoginView;
use crate::views::record::GetRecordView;

mod interfaces;
pub mod models;
pub mod views;

const APP_NAME: &str = "Admin panel";

pub struct AdminPanel;

impl AdminPanel {
    pub async fn register() {
        Project::register_app(Self::build_app()).await
    }

    pub fn build_app() -> App {
        let admin_models = vec![Model::new(Box::<AdminUser>::default(), AdminUser::scheme())];

        let mut patterns = UrlPatterns::default();
        patterns.add(vec![
            ("/admin/login", Box::new(LoginView)),
            ("/admin/settings", Box::new(GetSettingsView)),
            (
                "/admin/apps",
                Box::new(GetAppsView),
            ),
            ("/admin/record", Box::new(GetRecordView)),
        ]);

        App::new(APP_NAME, patterns, admin_models)
    }
}
