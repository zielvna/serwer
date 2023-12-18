use super::cookie::Cookie;
use crate::enums::SerwerError;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Cookies {
    cookies: BTreeMap<String, Cookie>,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            cookies: BTreeMap::new(),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut cookies = BTreeMap::new();

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

    pub fn set_cookie(&mut self, key: &str, value: Cookie) {
        self.cookies.insert(String::from(key), value);
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

        let mut cookies = Cookies::new();
        cookies.set_cookie("id", Cookie::from_string("id=1").unwrap());

        assert_eq!(result, Ok(cookies.clone()));

        let string = &String::from("id=1; name=John");
        let result = Cookies::from_string(string);

        cookies.set_cookie("name", Cookie::from_string("name=John").unwrap());

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
        assert_eq!(result, Ok(Cookies::new()));
    }

    #[test]
    fn test_to_bytes() {
        let mut cookies = Cookies::new();
        cookies.set_cookie(
            "id",
            Cookie::new("id", "1")
                .set_expires("Mon, 18 Dec 2023 06:11:00 GMT")
                .set_domain("localhost")
                .set_path("/")
                .set_secure(true),
        );
        cookies.set_cookie(
            "name",
            Cookie::new("name", "John")
                .set_max_age(86400)
                .set_http_only(true)
                .set_same_site("Strict"),
        );
        let result = cookies.to_bytes();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            String::from("Set-Cookie: id=1; Expires=Mon, 18 Dec 2023 06:11:00 GMT; Domain=localhost; Path=/; Secure\r\nSet-Cookie: name=John; Max-Age=86400; HttpOnly; SameSite=Strict\r\n")
        )
    }
}
