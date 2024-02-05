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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let params = Params::new();
        assert_eq!(params.params.len(), 0);
    }

    #[test]
    fn test_param() {
        let mut params = Params::new();
        params.set_param("user", "1");
        assert_eq!(params.param("user").unwrap(), "1");
    }

    #[test]
    fn test_set_param() {
        let mut params = Params::new();
        params.set_param("user", "1");
        assert_eq!(params.params.len(), 1);
        assert_eq!(params.param("user").unwrap(), "1");
    }
}
