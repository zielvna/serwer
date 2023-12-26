use crate::{decode, SerwerError};

const ALLOWED_CHARACTERS: &str =
    "%-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";

const ALLOWED_CHARACTERS_WITH_RESERVED: &str =
    "!$&'()*+,-.0123456789:;=@ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    string: String,
    is_param: bool,
}

impl Segment {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let mut parsed_string = string.to_string();

        let is_param = parsed_string.starts_with("<") && parsed_string.ends_with(">");

        if is_param {
            parsed_string = parsed_string[1..parsed_string.len() - 1].to_string();
        }

        if !parsed_string
            .chars()
            .all(|c| ALLOWED_CHARACTERS.contains(c))
        {
            return Err(SerwerError::InvalidPathSegmentCharacters(String::from(
                string,
            )));
        }

        parsed_string = decode(&parsed_string)?;

        if !parsed_string
            .chars()
            .all(|c| ALLOWED_CHARACTERS_WITH_RESERVED.contains(c))
        {
            return Err(SerwerError::InvalidPathSegmentCharacters(String::from(
                string,
            )));
        }

        Ok(Self {
            string: parsed_string,
            is_param,
        })
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
            result.unwrap(),
            Segment {
                string: String::from("user"),
                is_param: false
            }
        );

        let string = &String::from("<user>");
        let result = Segment::from_string(string);
        assert_eq!(
            result.unwrap(),
            Segment {
                string: String::from("user"),
                is_param: true
            }
        );
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Segment::from_string(string);
        assert_eq!(
            result.unwrap(),
            Segment {
                string: String::from(""),
                is_param: false
            }
        );

        let string = &String::from("<>");
        let result = Segment::from_string(string);
        assert_eq!(
            result.unwrap(),
            Segment {
                string: String::from(""),
                is_param: true
            }
        );
    }

    #[test]
    fn test_from_string_characters() {
        let string = &String::from("us-er");
        let result = Segment::from_string(string);
        assert_eq!(
            result.unwrap(),
            Segment {
                string: String::from("us-er"),
                is_param: false
            }
        );

        let string = &String::from("us%21er");
        let result = Segment::from_string(string);
        assert_eq!(
            result.unwrap(),
            Segment {
                string: String::from("us!er"),
                is_param: false
            }
        );

        let string = &String::from("us%20er");
        let result = Segment::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidPathSegmentCharacters(error_string)) if &error_string == "us%20er"
        ));

        let string = &String::from("us#er");
        let result = Segment::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidPathSegmentCharacters(error_string)) if &error_string == "us#er"
        ));
    }

    #[test]
    fn test_from_string_invalid_param_chars() {
        let string = &String::from("<user");
        let result = Segment::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidPathSegmentCharacters(error_string)) if &error_string == "<user"
        ));

        let string = &String::from("user>");
        let result = Segment::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidPathSegmentCharacters(error_string)) if &error_string == "user>"
        ));

        let string = &String::from("<use>r");
        let result = Segment::from_string(string);
        assert!(matches!(
            result,
            Err(SerwerError::InvalidPathSegmentCharacters(error_string)) if &error_string == "<use>r"
        ));
    }
}
