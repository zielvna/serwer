use super::{Params, Segment};
use crate::enums::SerwerError;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    string: String,
    segments: Vec<Segment>,
    segments_length: usize,
}

impl Path {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let string = string.to_string();

        if !string.starts_with("/")
            || (string.ends_with("/") && string.len() > 1)
            || string.contains("//")
        {
            return Err(SerwerError::InvalidPathSlashes);
        }

        let mut segments: Vec<Segment> = vec![];
        let mut segments_length = 0;

        if string != "/" {
            let parts: Vec<String> = string[1..string.len()]
                .split("/")
                .map(String::from)
                .collect();

            for part in parts.iter() {
                let segment = Segment::from_string(part)?;
                segments.push(segment);
                segments_length += 1;
            }
        }

        Ok(Self {
            string,
            segments,
            segments_length,
        })
    }

    pub fn matches(&self, other_path: &Path) -> (bool, Option<Params>) {
        let mut params = Params::new();

        if self.segments_length != other_path.segments_length {
            return (false, None);
        }

        for i in 0..self.segments_length {
            let mut is_param = false;

            if self.segments[i].is_param() && other_path.segments[i].is_param() {
                return (false, None);
            }

            if self.segments[i].is_param() {
                is_param = true;
                params.set_param(
                    self.segments[i].get_string(),
                    other_path.segments[i].get_string(),
                );
            }

            if other_path.segments[i].is_param() {
                is_param = true;
                params.set_param(
                    other_path.segments[i].get_string(),
                    self.segments[i].get_string(),
                );
            }

            if !is_param && self.segments[i] != other_path.segments[i] {
                return (false, None);
            }
        }

        (true, Some(params))
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let string = &String::from("/user");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user"),
                segments: vec![Segment::from_string("user").unwrap()],
                segments_length: 1,
            })
        );

        let string = &String::from("/user/<id>");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user/<id>"),
                segments: vec![
                    Segment::from_string("user").unwrap(),
                    Segment::from_string("<id>").unwrap()
                ],
                segments_length: 2,
            })
        );
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("/");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/"),
                segments: vec![],
                segments_length: 0,
            })
        );
    }

    #[test]
    fn test_from_string_invalid_slashes() {
        let string = &String::from("user");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSlashes));

        let string = &String::from("/user/");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSlashes));

        let string = &String::from("/user//<id>");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSlashes));
    }

    #[test]
    fn test_matches() {
        let path = Path::from_string(&String::from("/user")).unwrap();
        let other_path = Path::from_string(&String::from("/user")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (true, Some(Params::new())));

        let mut params = Params::new();
        params.set_param("user_id", "1");
        params.set_param("post_id", "2");

        let path = Path::from_string(&String::from("/user/1/post/2")).unwrap();
        let other_path =
            Path::from_string(&String::from("/user/<user_id>/post/<post_id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (true, Some(params.clone())));
        let result = other_path.matches(&path);
        assert_eq!(result, (true, Some(params.clone())));
    }

    #[test]
    fn test_matches_not() {
        let path = Path::from_string(&String::from("/user")).unwrap();
        let other_path = Path::from_string(&String::from("/profile")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (false, None));

        let path = Path::from_string(&String::from("/user")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (false, None));

        let path = Path::from_string(&String::from("/user/1/post/2")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (false, None));
    }

    #[test]
    fn test_matches_param_in_both_paths() {
        let path = Path::from_string(&String::from("/user/<profile>")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (false, None));

        let path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (false, None));
    }
}
