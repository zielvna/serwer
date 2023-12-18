use crate::enums::SerwerError;
use std::collections::HashMap;

const NAME_ALLOWED_CHARACTERS: &str =
    "!#$%&'*+-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ^_`abcdefghijklmnopqrstuvwxyz|~";

const VALUE_ALLOWED_CHARACTERS: &str =
    "!#$%&'()*+-./0123456789:<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Debug, Clone, PartialEq)]
pub struct Cookies {
    cookies: HashMap<String, String>,
}

impl Cookies {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut cookies = HashMap::new();

        if string.is_empty() {
            return Ok(Self { cookies });
        }

        let parts: Vec<&str> = string.split("; ").collect();

        for part in parts.iter() {
            let (name, value) = part.split_once('=').ok_or(SerwerError::InvalidCookie)?;

            if name.is_empty() || value.is_empty() {
                return Err(SerwerError::InvalidCookie);
            }

            if !name.chars().all(|c| NAME_ALLOWED_CHARACTERS.contains(c)) {
                return Err(SerwerError::InvalidCookieCharacters);
            }

            if value.starts_with("\"") && value.ends_with("\"") {
                if !value[1..value.len() - 1]
                    .chars()
                    .all(|c| VALUE_ALLOWED_CHARACTERS.contains(c))
                {
                    return Err(SerwerError::InvalidCookieCharacters);
                }
            } else {
                if !value.chars().all(|c| VALUE_ALLOWED_CHARACTERS.contains(c)) {
                    return Err(SerwerError::InvalidCookieCharacters);
                }
            }

            cookies.insert(String::from(name), String::from(value));
        }

        Ok(Self { cookies })
    }

    pub fn get_cookie(&self, key: &str) -> Option<&String> {
        self.cookies.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("id=1");
        let result = Cookies::from_string(string);

        let mut hashmap = HashMap::new();
        hashmap.insert(String::from("id"), String::from("1"));
        let cookies = Cookies {
            cookies: hashmap.clone(),
        };

        assert_eq!(result, Ok(cookies.clone()));

        let string = &String::from("id=1; name=John");
        let result = Cookies::from_string(string);

        hashmap.insert(String::from("name"), String::from("John"));
        let cookies = Cookies {
            cookies: hashmap.clone(),
        };

        assert_eq!(result, Ok(cookies.clone()));

        hashmap.insert(String::from("name"), String::from("\"John\""));
        let cookies = Cookies { cookies: hashmap };

        let string = &String::from("id=1; name=\"John\"");
        let result = Cookies::from_string(string);
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
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));
    }

    #[test]
    fn test_from_string_invalid_characters() {
        let string = &String::from("name=Jo,hn");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("na@me=John");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));
    }

    #[test]
    fn test_from_string_invalid_double_quotes() {
        let string = &String::from("name=\"John");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("name=John\"");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("name=\"Joh\"n");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Cookies::from_string(string);
        assert_eq!(
            result,
            Ok(Cookies {
                cookies: HashMap::new()
            })
        );

        let string = &String::from("=");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));
    }

    #[test]
    fn test_from_string_invalid_cookie() {
        let string = &String::from("id=");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));

        let string = &String::from("=1");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));

        let string = &String::from("id=1; name=");
        let result = Cookies::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));
    }
}
