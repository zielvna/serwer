use crate::SerwerError;

const NAME_ALLOWED_CHARACTERS: &str =
    "!#$%&'*+-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ^_`abcdefghijklmnopqrstuvwxyz|~";

const VALUE_ALLOWED_CHARACTERS: &str =
    "!#$%&'()*+-./0123456789:<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Debug, Clone, PartialEq)]
pub struct Cookie {
    name: String,
    value: String,
    expires: Option<String>,
    max_age: Option<u64>,
    domain: Option<String>,
    path: Option<String>,
    secure: bool,
    http_only: bool,
    same_site: Option<String>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: String::from(name),
            value: String::from(value),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let (name, value) = string.split_once('=').ok_or(SerwerError::InvalidCookie)?;

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

        Ok(Self {
            name: String::from(name),
            value: String::from(value),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: None,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_expires(&self) -> Option<&String> {
        self.expires.as_ref()
    }

    pub fn get_max_age(&self) -> Option<&u64> {
        self.max_age.as_ref()
    }

    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn get_secure(&self) -> bool {
        self.secure
    }

    pub fn get_http_only(&self) -> bool {
        self.http_only
    }

    pub fn get_same_site(&self) -> Option<&String> {
        self.same_site.as_ref()
    }

    pub fn set_expires(mut self, expires: &str) -> Self {
        self.expires = Some(String::from(expires));
        self
    }

    pub fn set_max_age(mut self, max_age: u64) -> Self {
        self.max_age = Some(max_age);
        self
    }

    pub fn set_domain(mut self, domain: &str) -> Self {
        self.domain = Some(String::from(domain));
        self
    }

    pub fn set_path(mut self, path: &str) -> Self {
        self.path = Some(String::from(path));
        self
    }

    pub fn set_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    pub fn set_http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    pub fn set_same_site(mut self, same_site: &str) -> Self {
        self.same_site = Some(String::from(same_site));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("id=1");
        let result = Cookie::from_string(string);
        assert_eq!(result, Ok(Cookie::new("id", "1")));

        let string = &String::from("name=\"John\"");
        let result = Cookie::from_string(string);
        assert_eq!(result, Ok(Cookie::new("name", "\"John\"")));
    }

    #[test]
    fn test_from_string_invalid_characters() {
        let string = &String::from("name=Jo,hn");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("na@me=John");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));
    }

    #[test]
    fn test_from_string_invalid_double_quotes() {
        let string = &String::from("name=\"John");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("name=John\"");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));

        let string = &String::from("name=\"Joh\"n");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookieCharacters));
    }

    #[test]
    fn test_from_string_invalid_cookie() {
        let string = &String::from("=John");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));

        let string = &String::from("name=");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));

        let string = &String::from("=");
        let result = Cookie::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidCookie));
    }

    #[test]
    fn test_cookie_builder() {
        let cookie = Cookie::new("id", "1")
            .set_expires("Mon, 18 Dec 2023 06:11:00 GMT")
            .set_max_age(86400)
            .set_domain("localhost")
            .set_path("/")
            .set_secure(true)
            .set_http_only(true)
            .set_same_site("Strict");

        assert_eq!(cookie.get_name(), "id");
        assert_eq!(
            cookie.expires,
            Some(String::from("Mon, 18 Dec 2023 06:11:00 GMT"))
        );
        assert_eq!(cookie.max_age, Some(86400));
        assert_eq!(cookie.domain, Some(String::from("localhost")));
        assert_eq!(cookie.path, Some(String::from("/")));
        assert_eq!(cookie.secure, true);
        assert_eq!(cookie.http_only, true);
        assert_eq!(cookie.same_site, Some(String::from("Strict")));
    }
}
