use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Cookie {
    data: HashMap<String, String>,
}

impl Cookie {
    pub fn from_string<T: ToString>(string: T) -> Self {
        let mut res = Self::default();

        for cook in string.to_string().split("; ") {
            let split: Vec<&str> = cook.split('=').collect();
            if split.len() != 2 {
                continue;
            }

            res.add(split[0], split[1]);
        }

        res
    }

    pub fn format_to_string(&self) -> String {
        let mut res = Vec::new();

        for (key, val) in &self.data {
            res.push(format!("{key}={val}"));
        }

        res.join(", ")
    }

    pub fn add<T1: ToString, T2: ToString>(&mut self, key: T1, val: T2) {
        self.data.insert(key.to_string(), val.to_string());
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
