use crate::urlpatterns::UrlPatterns;

pub struct App {
    name: String,
    patterns: UrlPatterns,
}

impl App {
    pub fn new<T: ToString>(name: T, patterns: UrlPatterns) -> Self {
        Self {
            name: name.to_string(),
            patterns,
        }
    }

    pub fn url_patters(&self) -> UrlPatterns {
        self.patterns.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
