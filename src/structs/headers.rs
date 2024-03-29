use crate::SerwerError;
use std::collections::BTreeMap;

const NAME_ALLOWED_CHARACTERS: &str =
    "!#$%&'*+-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ^_`abcdefghijklmnopqrstuvwxyz|~";

const VALUE_ALLOWED_CHARACTERS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Debug, Clone, PartialEq)]
pub struct Headers {
    headers: BTreeMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            headers: BTreeMap::new(),
        }
    }

    pub fn set_header_from_string(&mut self, string: &str) -> Result<(), SerwerError> {
        let (name, value) = string
            .split_once(':')
            .ok_or(SerwerError::InvalidHeader(String::from(string)))?;
        let value = value.trim();

        if name.is_empty() {
            return Err(SerwerError::InvalidHeader(String::from(string)));
        }

        if !name.chars().all(|c| NAME_ALLOWED_CHARACTERS.contains(c)) {
            return Err(SerwerError::InvalidHeaderCharacters(String::from(string)));
        }

        if !value.chars().all(|c| VALUE_ALLOWED_CHARACTERS.contains(c)) {
            return Err(SerwerError::InvalidHeaderCharacters(String::from(string)));
        }

        self.headers
            .insert(name.to_lowercase(), String::from(value));

        Ok(())
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers
            .insert(name.to_lowercase(), String::from(value));
    }

    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name.to_lowercase().as_str())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        for (name, value) in self.headers.iter() {
            bytes.extend(name.as_bytes());
            bytes.extend(b": ");
            bytes.extend(value.as_bytes());
            bytes.extend(b"\r\n");
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_header_from_string() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host: localhost:80");
        assert_eq!(result.unwrap(), ());
        assert_eq!(headers.header("Host"), Some(&String::from("localhost:80")));

        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host:");
        assert_eq!(result.unwrap(), ());
        assert_eq!(headers.header("Host"), Some(&String::from("")));

        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host:   local  host:80  ");
        assert_eq!(result.unwrap(), ());
        assert_eq!(
            headers.header("Host"),
            Some(&String::from("local  host:80"))
        );
    }

    #[test]
    fn test_set_header_from_string_invalid_characters() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Ho@st: localhost:80");
        assert!(
            matches!(result, Err(SerwerError::InvalidHeaderCharacters(error_string)) if &error_string == "Ho@st: localhost:80")
        );
        assert_eq!(headers.header("Ho@st"), None);

        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Host: localh€ost:80");
        assert!(
            matches!(result, Err(SerwerError::InvalidHeaderCharacters(error_string)) if &error_string == "Host: localh€ost:80")
        );
        assert_eq!(headers.header("Host"), None);
    }

    #[test]
    fn test_set_header_from_string_invalid_header() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("Connection keep-alive");
        assert!(
            matches!(result, Err(SerwerError::InvalidHeader(error_string)) if &error_string == "Connection keep-alive")
        );
        assert_eq!(headers.header("Host"), None);

        let mut headers = Headers::new();
        let result = headers.set_header_from_string(": localhost:80");
        assert!(
            matches!(result, Err(SerwerError::InvalidHeader(error_string)) if &error_string == ": localhost:80")
        );
        assert_eq!(headers.header(""), None);
    }

    #[test]
    fn test_set_header_from_string_empty() {
        let mut headers = Headers::new();
        let result = headers.set_header_from_string("");
        assert!(
            matches!(result, Err(SerwerError::InvalidHeader(error_string)) if &error_string == "")
        );
        assert_eq!(headers.header(""), None);

        let mut headers = Headers::new();
        let result = headers.set_header_from_string(":");
        assert!(
            matches!(result, Err(SerwerError::InvalidHeader(error_string)) if &error_string == ":")
        );
        assert_eq!(headers.header(""), None);
    }

    #[test]
    fn test_to_bytes() {
        let mut headers = Headers::new();
        headers.set_header("Host", "localhost:80");
        headers.set_header("Connection", "keep-alive");
        let result = headers.to_bytes();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            String::from("connection: keep-alive\r\nhost: localhost:80\r\n")
        )
    }
}
