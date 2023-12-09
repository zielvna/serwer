use crate::enums::SerwerError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Path {
    string: String,
    parts: Vec<String>,
    map: Vec<bool>,
    length: usize,
}

impl Path {
    pub fn from_string(string: String) -> Result<Self, SerwerError> {
        if !string.starts_with("/") || string.ends_with("/") || string.len() < 2 {
            return Err(SerwerError::InvalidPathSlashes);
        }

        let mut string = string;
        string.remove(0);

        let parts: Vec<&str> = string.split("/").collect();
        let mut parsed_parts: Vec<String> = vec![];
        let mut map: Vec<bool> = vec![];

        let mut length = 0;

        for part in parts.iter() {
            let (parsed_part, is_param) = validate_part(part)?;
            parsed_parts.push(parsed_part);
            map.push(is_param);
            length += 1;
        }

        Ok(Self {
            string: String::from(string),
            parts: parsed_parts,
            map,
            length,
        })
    }

    pub fn matches_to(&self, other_path: &Path) -> (bool, Option<HashMap<String, String>>) {
        let mut params: HashMap<String, String> = HashMap::new();

        if self.length != other_path.length {
            return (false, None);
        }

        for i in 0..self.length {
            let mut is_param = false;

            if self.map[i] {
                is_param = true;
                params.insert(self.parts[i].clone(), other_path.parts[i].clone());
            }

            if other_path.map[i] {
                is_param = true;
                params.insert(other_path.parts[i].clone(), self.parts[i].clone());
            }

            if !is_param && self.parts[i] != other_path.parts[i] {
                return (false, None);
            }
        }

        (true, Some(params))
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }
}

const ALLOWED_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";

fn validate_part(part: &str) -> Result<(String, bool), SerwerError> {
    let (part, is_param) = match part.starts_with("<") {
        true => {
            if !part.ends_with(">") || part.len() < 3 {
                return Err(SerwerError::InvalidPathPart);
            };

            let part_without_param_chars = &part[1..part.len() - 1];

            for char in part_without_param_chars.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidPathCharacters);
                }
            }

            (part_without_param_chars, true)
        }
        false => {
            if part.len() < 1 {
                return Err(SerwerError::InvalidPathPart);
            };

            for char in part.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidPathCharacters);
                }
            }

            (part, false)
        }
    };

    Ok((String::from(part), is_param))
}
