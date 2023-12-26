use crate::{decode, SerwerError};
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
                .ok_or(SerwerError::InvalidQueryParam(String::from(part)))?;
            let value = value.trim();

            if name.is_empty() || value.is_empty() {
                return Err(SerwerError::InvalidQueryParam(String::from(part)));
            }

            if !name.chars().all(|c| NAME_ALLOWED_CHARACTERS.contains(c)) {
                return Err(SerwerError::InvalidQueryParamCharacters(String::from(part)));
            }

            if !value.chars().all(|c| VALUE_ALLOWED_CHARACTERS.contains(c)) {
                return Err(SerwerError::InvalidQueryParamCharacters(String::from(part)));
            }

            let value = decode(value)?;

            if !value
                .chars()
                .all(|c| VALUE_ALLOWED_CHARACTERS_WITH_RESERVED.contains(c))
            {
                return Err(SerwerError::InvalidQueryParamCharacters(String::from(part)));
            }

            query_params.insert(String::from(name), String::from(value));
        }

        Ok(Self { query_params })
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

        assert_eq!(result.unwrap(), query_params.clone());

        let string = &String::from("id=1&name=John+Doe");
        let result = QueryParams::from_string(string);

        hashmap.insert(String::from("name"), String::from("John Doe"));
        let query_params = QueryParams {
            query_params: hashmap,
        };

        assert_eq!(result.unwrap(), query_params.clone());

        let string = &String::from("id=1&name=John%20Doe");
        let result = QueryParams::from_string(string);

        assert_eq!(result.unwrap(), query_params);
    }

    #[test]
    fn test_from_string_invalid_characters() {
        let string = &String::from("name=Jo€hn");
        let result = QueryParams::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidQueryParamCharacters(error_string)) if &error_string == "name=Jo€hn"
        ));

        let string = &String::from("na@me=John");
        let result = QueryParams::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidQueryParamCharacters(error_string)) if &error_string == "na@me=John"
        ));

        let string = &String::from("name=Jo%5Ehn");
        let result = QueryParams::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidQueryParamCharacters(error_string)) if &error_string == "name=Jo%5Ehn"
        ));

        let string = &String::from("name=Jo hn");
        let result = QueryParams::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidQueryParamCharacters(error_string)) if &error_string == "name=Jo hn"
        ));

        let string = &String::from("name=Jo%hn");
        let result = QueryParams::from_string(string);
        assert!(matches!(result, Err(SerwerError::ParseIntError(_))));
    }

    #[test]
    fn test_from_string_invalid_ampersands() {
        let string = &String::from("&id=1");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "")
        );

        let string = &String::from("id=1&");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "")
        );

        let string = &String::from("id=1&&name=John");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "")
        );
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "")
        );

        let string = &String::from("=");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "=")
        );
    }

    #[test]
    fn test_from_string_invalid_query_param() {
        let string = &String::from("id=");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "id=")
        );

        let string = &String::from("=1");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "=1")
        );

        let string = &String::from("id=1&name=");
        let result = QueryParams::from_string(string);
        assert!(
            matches!(result, Err(SerwerError::InvalidQueryParam(error_string)) if &error_string == "name=")
        );
    }
}
