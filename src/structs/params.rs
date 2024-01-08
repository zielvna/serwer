use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Params {
    params: HashMap<String, String>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn set_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }
}
