use crate::view::View;
use std::collections::HashMap;
use std::sync::Arc;

type Callback = Box<dyn View + Send + Sync>;

#[derive(Clone)]
pub struct UrlPatterns {
    patterns: HashMap<String, Arc<Callback>>,
}

impl UrlPatterns {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
        }
    }

    pub fn add<T: ToString>(&mut self, patterns: Vec<(T, Callback)>) {
        for (url, view) in patterns {
            self.patterns.insert(url.to_string(), Arc::new(view));
        }
    }

    pub fn find_pattern<T: ToString>(&self, url: T) -> Option<&Arc<Callback>> {
        self.patterns.get(&url.to_string())
    }

    pub fn merge_all(patters: Vec<Self>) -> Self {
        let mut res = Self::new();

        for pattern in patters {
            for (url, view) in pattern.patterns {
                if res.patterns.contains_key(&url) {
                    eprintln!("found the same patterns: {url}");
                    continue;
                }

                res.patterns.insert(url, view);
            }
        }

        res
    }
}
