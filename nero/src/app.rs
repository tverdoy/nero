use crate::urlpatterns::UrlPatterns;

pub trait App {
    fn name() -> &'static str;

    fn url_patterns() -> UrlPatterns;
}