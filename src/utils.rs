use crate::enums::SerwerError;

const ALLOWED_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn validate_path(path: &str) -> Result<(), SerwerError> {
    if path.starts_with("/") || path.ends_with("/") {
        return Err(SerwerError::InvalidPathSlashes);
    }

    let path_parts: Vec<&str> = path.split("/").collect();

    for path_part in path_parts.iter() {
        validate_path_part(path_part)?;
    }

    Ok(())
}

fn validate_path_part(path_part: &str) -> Result<(), SerwerError> {
    match path_part.starts_with("<") {
        true => {
            if !path_part.ends_with(">") || path_part.len() < 3 {
                return Err(SerwerError::InvalidPathPart);
            };

            for char in (&path_part[1..path_part.len() - 1]).chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidPathCharacters);
                }
            }
        }
        false => {
            if path_part.len() < 1 {
                return Err(SerwerError::InvalidPathPart);
            };

            for char in path_part.chars() {
                if !ALLOWED_CHARACTERS.contains(char) {
                    return Err(SerwerError::InvalidPathCharacters);
                }
            }
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{enums::SerwerError, utils::validate_path};

    #[test]
    fn path_validation() {
        let path = "user";
        let result = validate_path(path);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn path_validation_slash_at_the_start_or_end() {
        let path = "/user";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathSlashes));

        let path = "user/";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathSlashes));
    }

    #[test]
    fn path_validation_double_slash() {
        let path = "user//user";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathPart));
    }

    #[test]
    fn path_validation_invalid_char() {
        let path = "us`er";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathCharacters));
    }

    #[test]
    fn path_validation_param() {
        let path = "user/<user>";
        let result = validate_path(path);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn path_validation_no_param_name() {
        let path = "user/<>";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathPart));
    }

    #[test]
    fn path_validation_double_param_char() {
        let path = "user/<<user>";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathCharacters));
    }

    #[test]
    fn path_validation_param_char_in_the_middle() {
        let path = "user/<us>er";
        let result = validate_path(path);
        assert_eq!(result, Err(SerwerError::InvalidPathPart));
    }
}
