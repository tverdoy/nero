use nero::app::App;
use nero::project::Project;
use nero::urlpatterns::UrlPatterns;

mod models;
mod views;

pub async fn register() {
    let mut patterns = UrlPatterns::default();
    patterns.add(vec![("/home", Box::new(views::home::HomeView))]);

    Project::register_app(App::new("Messenger", patterns, models::build_models())).await;
}
