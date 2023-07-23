use crate::urlpatterns::UrlPatterns;
use std::rc::Rc;
use std::sync::Arc;

pub struct App {
    name: String,
    url_patterns: UrlPatterns,
}

impl App {
    pub fn new<T: ToString>(name: T, url_patterns: UrlPatterns) -> Self {
        Self {
            name: name.to_string(),
            url_patterns,
        }
    }

    pub fn url_patters(&self) -> UrlPatterns {
        self.url_patterns.clone()
    }
}
