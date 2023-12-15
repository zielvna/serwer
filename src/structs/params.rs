use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Params {
    pub params: HashMap<String, String>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        self.params.get(key).cloned()
    }

    pub fn get_params(&self) -> HashMap<String, String> {
        self.params.to_owned()
    }

    pub fn set_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }
}
