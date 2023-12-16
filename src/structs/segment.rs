use crate::enums::SerwerError;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    string: String,
    is_param: bool,
}

const ALLOWED_CHARACTERS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

impl Segment {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut string = string.to_string();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("user");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from("user"),
                is_param: false
            })
        );

        let string = &String::from("<user>");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from("user"),
                is_param: true
            })
        );
    }

    #[test]
    fn test_from_string_characters() {
        let string = &String::from("u_s-e.r~");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from("u_s-e.r~"),
                is_param: false
            })
        );

        let string = &String::from("u!s@e#r$");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyPathSegment));

        let string = &String::from("<>");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::EmptyPathSegment));
    }

    #[test]
    fn test_from_string_invalid_param_chars() {
        let string = &String::from("<user");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));

        let string = &String::from("<use>r");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));
    }
}
