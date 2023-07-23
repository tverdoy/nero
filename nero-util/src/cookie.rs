use std::collections::HashMap;

#[derive(Debug)]
pub struct Cookie {
    data: HashMap<String, String>,
}

impl Cookie {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn from_string<T: ToString>(string: T) -> Self {
        let mut res = Self::new();

        for cook in string.to_string().split("; ") {
            let split: Vec<&str> = cook.split('=').collect();
            if split.len() != 2 {
                continue;
            }

            res.add(split[0], split[1]);
        }

        res
    }

    pub fn add<T1: ToString, T2: ToString>(&mut self, key: T1, val: T2) {
        self.data.insert(key.to_string(), val.to_string());
    }
}
