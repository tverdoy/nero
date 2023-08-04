use nero::app::App;
use nero::urlpatterns::UrlPatterns;

mod models;
mod views;

pub fn build_app() -> App {
    let mut patterns = UrlPatterns::default();
    patterns.add(vec![("/home", Box::new(views::home::HomeView))]);

    App::new("Messenger", patterns, models::build_models())
}
