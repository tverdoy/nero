use nero::app::App;
use nero::db::model::Model;
use nero::urlpatterns::UrlPatterns;
use views::*;

use crate::interfaces::InterfaceApp;
use crate::models::admin_user::AdminUser;
use crate::views::settings::ViewGetSettings;
use nero::db::model::Manager;

mod interfaces;
pub mod models;
pub mod views;

const APP_NAME: &str = "Admin panel";

pub struct AdminPanel {
    apps: Vec<InterfaceApp>,
}

impl AdminPanel {
    pub fn new(apps: &[App]) -> Self {
        let mut self_apps = Vec::new();

        for app in apps {
            self_apps.push(InterfaceApp::from(app))
        }
        Self { apps: self_apps }
    }

    pub fn build_app(&mut self) -> App {
        let admin_models = vec![Model::new(Box::<AdminUser>::default(), AdminUser::scheme())];

        self.apps.push(InterfaceApp {
            name: APP_NAME.to_string(),
            schemes: admin_models
                .iter()
                .map(|model| model.scheme.clone())
                .collect(),
        });

        let mut patterns = UrlPatterns::default();
        patterns.add(vec![
            ("/admin/login", Box::new(login::LoginView)),
            ("/admin/settings", Box::new(ViewGetSettings)),
            (
                "/admin/apps",
                Box::new(apps::GetAppsView::new(self.apps.clone())),
            ),
        ]);

        App::new(APP_NAME, patterns, admin_models)
    }
}
