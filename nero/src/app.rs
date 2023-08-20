use crate::db::model::Model;
use crate::urlpatterns::UrlPatterns;
use serde::Serialize;

#[derive(Serialize)]
pub struct App {
    pub name: String,
    #[serde(skip)]
    pub patterns: UrlPatterns,
    pub models: Vec<Model>,
}

impl App {
    pub fn new<T: ToString>(name: T, patterns: UrlPatterns, models: Vec<Model>) -> Self {
        Self {
            name: name.to_string(),
            patterns,
            models,
        }
    }
}
