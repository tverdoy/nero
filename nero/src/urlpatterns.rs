use std::collections::HashMap;
use std::sync::Arc;

use crate::app::App;
use crate::view::View;

pub type Callback = Box<dyn View + Send + Sync>;

#[derive(Clone, Default)]
pub struct UrlPatterns {
    patterns: HashMap<String, Arc<Callback>>,
}

impl UrlPatterns {
    pub fn add<T: ToString>(&mut self, patterns: Vec<(T, Callback)>) {
        for (url, view) in patterns {
            self.patterns.insert(url.to_string(), Arc::new(view));
        }
    }

    pub fn add_one<T: ToString>(&mut self, url: T, view: Callback) {
        self.patterns.insert(url.to_string(), Arc::new(view));
    }

    pub fn find_pattern<T: ToString>(&self, url: T) -> Option<&Arc<Callback>> {
        self.patterns.get(&url.to_string())
    }

    pub fn print_all_pattern(apps: &[App]) {
        for app in apps {
            for (url, _) in app.url_patters().patterns {
                println!("{url}")
            }
        }
    }
}
