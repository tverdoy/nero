use crate::db::model::Model;
use crate::urlpatterns::UrlPatterns;

pub struct App {
    name: String,
    patterns: UrlPatterns,
    models: Vec<Model>,
}

impl App {
    pub fn new<T: ToString>(name: T, patterns: UrlPatterns, models: Vec<Model>) -> Self {
        Self {
            name: name.to_string(),
            patterns,
            models,
        }
    }

    pub fn url_patters(&self) -> UrlPatterns {
        self.patterns.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn models(&self) -> &[Model] {
        &self.models
    }
}
