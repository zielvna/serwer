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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_param() {
        let mut params = Params::new();
        params.set_param("id", "1");

        assert_eq!(params.get_param("id"), Some(String::from("1")));
    }

    #[test]
    fn test_set_params() {
        let mut params = Params::new();
        params.set_param("id", "1");

        let mut another_params = Params::new();
        another_params.set_params(params.get_params());

        assert_eq!(another_params.get_param("id"), Some(String::from("1")));
    }
}
