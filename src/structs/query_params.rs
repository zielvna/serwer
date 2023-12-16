use crate::enums::SerwerError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryParams {
    query_params: HashMap<String, String>,
}

const ALLOWED_CHARACTERS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

impl QueryParams {
    pub fn new() -> Self {
        Self {
            query_params: HashMap::new(),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut query_params = Self::new();

        let parts: Vec<String> = string.split("&").map(String::from).collect();

        for part in parts.iter() {
            let parts: Vec<String> = part.split("=").map(String::from).collect();

            if parts.len() != 2 {
                return Err(SerwerError::InvalidPathQueryParam);
            }

            let key = &parts[0];
            let value = &parts[1];

            if key.is_empty() || value.is_empty() {
                return Err(SerwerError::EmptyPathQueryParam);
            }

            for char in key.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidPathQueryParamCharacters);
                }
            }

            for char in value.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidPathQueryParamCharacters);
                }
            }

            query_params.set_query_param(key, value);
        }

        Ok(query_params)
    }

    pub fn get_query_param(&self, key: &str) -> Option<String> {
        self.query_params.get(key).cloned()
    }

    pub fn get_query_params(&self) -> HashMap<String, String> {
        self.query_params.to_owned()
    }

    pub fn set_query_param(&mut self, key: &str, value: &str) {
        self.query_params.insert(key.to_string(), value.to_string());
    }

    pub fn set_query_params(&mut self, query_params: HashMap<String, String>) {
        self.query_params = query_params;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("id=1");
        let result = QueryParams::from_string(string);

        let mut query_params = QueryParams::new();
        query_params.set_query_param("id", "1");

        assert_eq!(result, Ok(query_params.clone()));

        let string = &String::from("id=1&name=John");
        let result = QueryParams::from_string(string);

        query_params.set_query_param("name", "John");

        assert_eq!(result, Ok(query_params));
    }

    #[test]
    fn test_from_string_invalid_ampersands() {
        let string = &String::from("&id=1");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));

        let string = &String::from("id=1&");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));

        let string = &String::from("id=1&&name=John");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));
    }

    #[test]
    fn test_from_string_invalid_query_param() {
        let string = &String::from("id=");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyPathQueryParam));

        let string = &String::from("=1");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyPathQueryParam));

        let string = &String::from("id=1&name=");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyPathQueryParam));
    }

    #[test]
    fn test_set_param() {
        let mut query_params = QueryParams::new();
        query_params.set_query_param("id", "1");

        assert_eq!(query_params.get_query_param("id"), Some(String::from("1")));
    }

    #[test]
    fn test_set_params() {
        let mut query_params = QueryParams::new();
        query_params.set_query_param("id", "1");

        let mut another_query_params = QueryParams::new();
        another_query_params.set_query_params(query_params.get_query_params());

        assert_eq!(
            another_query_params.get_query_param("id"),
            Some(String::from("1"))
        );
    }
}
