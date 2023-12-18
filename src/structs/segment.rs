use crate::enums::SerwerError;

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
        let mut string = string.to_string();

        let is_param = string.starts_with("<") && string.ends_with(">");

        if is_param {
            string = string[1..string.len() - 1].to_string();
        }

        if !string.chars().all(|c| ALLOWED_CHARACTERS.contains(c)) {
            return Err(SerwerError::InvalidPathSegmentCharacters);
        }

        string = Self::decode(&string)?;

        if !string
            .chars()
            .all(|c| ALLOWED_CHARACTERS_WITH_RESERVED.contains(c))
        {
            return Err(SerwerError::InvalidPathSegmentCharacters);
        }

        Ok(Self { string, is_param })
    }

    fn decode(string: &str) -> Result<String, SerwerError> {
        let mut result = String::new();
        let mut chars = string.chars();

        while let Some(char) = chars.next() {
            if char == '%' {
                let mut hex = String::new();

                hex.push(chars.next().ok_or(SerwerError::PathSegmentDecodeError)?);
                hex.push(chars.next().ok_or(SerwerError::PathSegmentDecodeError)?);

                let decoded = u8::from_str_radix(&hex, 16)
                    .map_err(|_| SerwerError::PathSegmentDecodeError)?
                    as char;

                result.push(decoded as char);
            } else {
                result.push(char);
            }
        }

        let result = result.replace("+", " ");

        Ok(result)
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
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from(""),
                is_param: false
            })
        );

        let string = &String::from("<>");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from(""),
                is_param: true
            })
        );
    }

    #[test]
    fn test_from_string_characters() {
        let string = &String::from("us-er");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from("us-er"),
                is_param: false
            })
        );

        let string = &String::from("us%21er");
        let result = Segment::from_string(string);
        assert_eq!(
            result,
            Ok(Segment {
                string: String::from("us!er"),
                is_param: false
            })
        );

        let string = &String::from("us%20er");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));

        let string = &String::from("us#er");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));
    }

    #[test]
    fn test_from_string_invalid_param_chars() {
        let string = &String::from("<user");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));

        let string = &String::from("user>");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));

        let string = &String::from("<use>r");
        let result = Segment::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSegmentCharacters));
    }
}
