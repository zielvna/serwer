use crate::enums::SerwerError;
use std::collections::HashMap;

const NAME_ALLOWED_CHARACTERS: &str =
    "-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";

const VALUE_ALLOWED_CHARACTERS: &str =
    "%+-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";

const VALUE_ALLOWED_CHARACTERS_WITH_RESERVED: &str =
    " !#$&'()*+,-./0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]_abcdefghijklmnopqrstuvwxyz~";

#[derive(Debug, Clone, PartialEq)]
pub struct QueryParams {
    query_params: HashMap<String, String>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            query_params: HashMap::new(),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut query_params = HashMap::new();

        let parts: Vec<String> = string.split("&").map(String::from).collect();

        for part in parts.iter() {
            let (name, value) = part
                .split_once('=')
                .ok_or(SerwerError::InvalidPathQueryParam)?;
            let value = value.trim();

            if name.is_empty() || value.is_empty() {
                return Err(SerwerError::InvalidPathQueryParam);
            }

            if !name.chars().all(|c| NAME_ALLOWED_CHARACTERS.contains(c)) {
                return Err(SerwerError::InvalidPathQueryParamCharacters);
            }

            if !value.chars().all(|c| VALUE_ALLOWED_CHARACTERS.contains(c)) {
                return Err(SerwerError::InvalidPathQueryParamCharacters);
            }

            let value = Self::decode(value)?;

            if !value
                .chars()
                .all(|c| VALUE_ALLOWED_CHARACTERS_WITH_RESERVED.contains(c))
            {
                return Err(SerwerError::InvalidPathQueryParamCharacters);
            }

            query_params.insert(String::from(name), String::from(value));
        }

        Ok(Self { query_params })
    }

    fn decode(string: &str) -> Result<String, SerwerError> {
        let mut result = String::new();
        let mut chars = string.chars();

        while let Some(char) = chars.next() {
            if char == '%' {
                let mut hex = String::new();

                hex.push(chars.next().ok_or(SerwerError::PathQueryParamDecodeError)?);
                hex.push(chars.next().ok_or(SerwerError::PathQueryParamDecodeError)?);

                let decoded = u8::from_str_radix(&hex, 16)
                    .map_err(|_| SerwerError::PathQueryParamDecodeError)?
                    as char;

                result.push(decoded as char);
            } else {
                result.push(char);
            }
        }

        let result = result.replace("+", " ");

        Ok(result)
    }

    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("id=1");
        let result = QueryParams::from_string(string);

        let mut hashmap = HashMap::new();
        hashmap.insert(String::from("id"), String::from("1"));
        let query_params = QueryParams {
            query_params: hashmap.clone(),
        };

        assert_eq!(result, Ok(query_params.clone()));

        let string = &String::from("id=1&name=John+Doe");
        let result = QueryParams::from_string(string);

        hashmap.insert(String::from("name"), String::from("John Doe"));
        let query_params = QueryParams {
            query_params: hashmap,
        };

        assert_eq!(result, Ok(query_params.clone()));

        let string = &String::from("id=1&name=John%20Doe");
        let result = QueryParams::from_string(string);

        assert_eq!(result, Ok(query_params));
    }

    #[test]
    fn test_from_string_invalid_characters() {
        let string = &String::from("name=Jo€hn");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParamCharacters));

        let string = &String::from("na@me=John");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParamCharacters));

        let string = &String::from("name=Jo%5Ehn");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParamCharacters));

        let string = &String::from("name=Jo hn");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParamCharacters));

        let string = &String::from("name=Jo%hn");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::PathQueryParamDecodeError));
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

        let string = &String::from("=");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));
    }

    #[test]
    fn test_from_string_invalid_query_param() {
        let string = &String::from("id=");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));

        let string = &String::from("=1");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));

        let string = &String::from("id=1&name=");
        let result = QueryParams::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));
    }
}
