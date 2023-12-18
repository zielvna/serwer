use super::cookie::Cookie;
use crate::enums::SerwerError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Cookies {
    cookies: HashMap<String, Cookie>,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut cookies = HashMap::new();

        if string.is_empty() {
            return Ok(Self { cookies });
        }

        let parts: Vec<&str> = string.split("; ").collect();

        for part in parts.iter() {
            let cookie = Cookie::from_string(part)?;

            cookies.insert(cookie.get_name().clone(), cookie);
        }

        Ok(Self { cookies })
    }

    pub fn get_cookie(&self, key: &str) -> Option<&Cookie> {
        self.cookies.get(key)
    }

    pub fn set_cookie(&mut self, key: String, value: Cookie) {
        self.cookies.insert(key, value);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        for (name, cookie) in self.cookies.iter() {
            bytes.extend(b"Set-Cookie: ");
            bytes.extend(name.as_bytes());
            bytes.extend(b"=");
            bytes.extend(cookie.get_value().as_bytes());
            if let Some(expires) = cookie.get_expires() {
                bytes.extend(b"; Expires=");
                bytes.extend(expires.as_bytes());
            }
            if let Some(max_age) = cookie.get_max_age() {
                bytes.extend(b"; Max-Age=");
                bytes.extend(max_age.to_string().as_bytes());
            }
            if let Some(domain) = cookie.get_domain() {
                bytes.extend(b"; Domain=");
                bytes.extend(domain.as_bytes());
            }
            if let Some(path) = cookie.get_path() {
                bytes.extend(b"; Path=");
                bytes.extend(path.as_bytes());
            }
            if cookie.get_secure() {
                bytes.extend(b"; Secure");
            }
            if cookie.get_http_only() {
                bytes.extend(b"; HttpOnly");
            }
            if let Some(same_site) = cookie.get_same_site() {
                bytes.extend(b"; SameSite=");
                bytes.extend(same_site.as_bytes());
            }
            bytes.extend(b"\r\n");
        }

        bytes
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
        hashmap.insert(String::from("id"), Cookie::from_string("id=1").unwrap());
        let cookies = Cookies {
            cookies: hashmap.clone(),
        };

        assert_eq!(result, Ok(cookies.clone()));

        let string = &String::from("id=1; name=John");
        let result = Cookies::from_string(string);

        hashmap.insert(
            String::from("name"),
            Cookie::from_string("name=John").unwrap(),
        );
        let cookies = Cookies {
            cookies: hashmap.clone(),
        };

        assert_eq!(result, Ok(cookies.clone()));
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
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Cookies::from_string(string);
        assert_eq!(
            result,
            Ok(Cookies {
                cookies: HashMap::new()
            })
        );
    }
}
