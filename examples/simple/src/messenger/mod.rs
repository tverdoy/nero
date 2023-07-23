mod views;

use nero::app::App;
use nero::urlpatterns::UrlPatterns;
use nero::view::View;
use std::sync::Arc;

pub fn build_app() -> App {
    let mut patterns = UrlPatterns::new();
    patterns.add(vec![("/home", Box::new(views::home::HomeView))]);

    App::new("messenger", patterns)
}
