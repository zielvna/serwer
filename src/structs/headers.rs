use crate::enums::SerwerError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Headers {
    headers: HashMap<String, String>,
}

const NAME_ALLOWED_CHARACTERS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";

const VALUE_ALLOWED_CHARACTERS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_ :;.,\\/\"'?!(){}[]@<>=-+*#$&`|~^%";

impl Headers {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn set_header_from_string(&mut self, string: &str) -> Result<(), SerwerError> {
        let index = string.find(":").ok_or(SerwerError::InvalidHeader)?;
        let (name, value) = string.split_at(index);

        if !value.starts_with(": ") {
            return Err(SerwerError::InvalidHeader);
        }

        let value = &value[2..];

        if name.is_empty() || value.is_empty() {
            return Err(SerwerError::EmptyHeader);
        }

        for char in name.chars() {
            if !NAME_ALLOWED_CHARACTERS.contains(char) {
                return Err(SerwerError::InvalidHeaderCharacters);
            }
        }

        for char in value.chars() {
            if !VALUE_ALLOWED_CHARACTERS.contains(char) {
                return Err(SerwerError::InvalidHeaderCharacters);
            }
        }

        self.headers.insert(String::from(name), String::from(value));

        Ok(())
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_header_from_string() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host: localhost:80");
        assert_eq!(result, Ok(()));
        assert_eq!(
            headers.get_header("Host"),
            Some(String::from("localhost:80"))
        );
    }

    #[test]
    fn test_set_header_from_string_invalid_header() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host:localhost:80");
        assert_eq!(result, Err(SerwerError::InvalidHeader));
        assert_eq!(headers.get_header("Host"), None);

        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host localhost:80");
        assert_eq!(result, Err(SerwerError::InvalidHeader));
        assert_eq!(headers.get_header("Host"), None);
    }

    #[test]
    fn test_set_header_from_string_characters() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host!: localhost:80");
        assert_eq!(result, Err(SerwerError::InvalidHeaderCharacters));
        assert_eq!(headers.get_header("Host!"), None);

        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host: localhost:80€");
        assert_eq!(result, Err(SerwerError::InvalidHeaderCharacters));
        assert_eq!(headers.get_header("Host"), None);
    }

    #[test]
    fn test_set_header_from_string_empty() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("");
        assert_eq!(result, Err(SerwerError::InvalidHeader));
        assert_eq!(headers.get_header(""), None);

        let mut headers = Headers::new();
        let result = headers.set_header_from_string(":");
        assert_eq!(result, Err(SerwerError::InvalidHeader));
        assert_eq!(headers.get_header(""), None);
    }
}
