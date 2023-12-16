use crate::enums::SerwerError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Cookies {
    cookies: HashMap<String, String>,
}

const ALLOWED_CHARACTERS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

impl Cookies {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut cookies = Self::new();

        let parts: Vec<String> = string.split("; ").map(String::from).collect();

        for part in parts.iter() {
            let parts: Vec<String> = part.split("=").map(String::from).collect();

            if parts.len() != 2 {
                return Err(SerwerError::InvalidCookie);
            }

            let key = &parts[0];
            let value = &parts[1];

            if key.is_empty() || value.is_empty() {
                return Err(SerwerError::EmptyCookie);
            }

            for char in key.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidCookieCharacters);
                }
            }

            for char in value.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidCookieCharacters);
                }
            }

            cookies.set_cookie(key, value);
        }

        Ok(cookies)
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        self.cookies.get(key).cloned()
    }

    pub fn get_cookies(&self) -> HashMap<String, String> {
        self.cookies.to_owned()
    }

    pub fn set_cookie(&mut self, key: &str, value: &str) {
        self.cookies.insert(key.to_string(), value.to_string());
    }

    pub fn set_cookies(&mut self, query_params: HashMap<String, String>) {
        self.cookies = query_params;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("id=1");
        let result = Cookies::from_string(string);

        let mut cookies = Cookies::new();
        cookies.set_cookie("id", "1");

        assert_eq!(result, Ok(cookies.clone()));

        let string = &String::from("id=1; name=John");
        let result = Cookies::from_string(string);

        cookies.set_cookie("name", "John");

        assert_eq!(result, Ok(cookies));
    }

    #[test]
    fn test_from_string_invalid_semicolon() {
        let string = &String::from(";id=1");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("id=1;");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("id=1;; name=John");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("id=1;name=John");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));
    }

    #[test]
    fn test_from_string_invalid_cookie() {
        let string = &String::from("id=");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyCookie));

        let string = &String::from("=1");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyCookie));

        let string = &String::from("id=1; name=");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyCookie));
    }

    #[test]
    fn test_set_param() {
        let mut cookies = Cookies::new();
        cookies.set_cookie("id", "1");

        assert_eq!(cookies.get_cookie("id"), Some(String::from("1")));
    }

    #[test]
    fn test_set_params() {
        let mut cookies = Cookies::new();
        cookies.set_cookie("id", "1");

        let mut another_cookies = Cookies::new();
        another_cookies.set_cookies(cookies.get_cookies());

        assert_eq!(another_cookies.get_cookie("id"), Some(String::from("1")));
    }
}
