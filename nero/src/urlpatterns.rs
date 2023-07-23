use std::collections::HashMap;
use crate::view::View;

pub struct UrlPatterns {
    patterns: HashMap<String, View>
}

impl UrlPatterns {
    pub fn new() -> Self {
        Self { patterns: HashMap::new() }
    }

    pub fn add<T: ToString>(&mut self, patterns: Vec<(T, View)>) {
        for (url, view) in patterns {
            self.patterns.insert(url.to_string(), view);
        }
    }
}