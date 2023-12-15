use crate::enums::SerwerError;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    string: String,
    is_param: bool,
}

const ALLOWED_CHARACTERS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

impl Segment {
    pub fn from_string(string: &String) -> Result<Segment, SerwerError> {
        let mut string = string.clone();

        let is_param = string.starts_with("<") && string.ends_with(">");

        if is_param {
            string.remove(0);
            string.pop();
        }

        if string.is_empty() {
            return Err(SerwerError::EmptyPathSegment);
        };

        for char in string.chars() {
            if !ALLOWED_CHARACTERS.contains(char) {
                return Err(SerwerError::InvalidPathSegmentCharacters);
            }
        }

        Ok(Self { string, is_param })
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }

    pub fn is_param(&self) -> bool {
        self.is_param
    }
}
