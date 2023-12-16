use super::{Params, QueryParams, Segment};
use crate::enums::SerwerError;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    string: String,
    segments: Vec<Segment>,
    segments_length: usize,
    query_params: QueryParams,
}

impl Path {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let string = string.to_string();

        let parts: Vec<String> = string.split("?").map(String::from).collect();
        let segments_string = &parts[0];

        let mut query_params = QueryParams::new();

        if string.contains("?") {
            let query_params_string = &parts[1];
            query_params = QueryParams::from_string(&query_params_string)?;
        }

        if !segments_string.starts_with("/")
            || (segments_string.ends_with("/") && segments_string.len() > 1)
            || segments_string.contains("//")
        {
            return Err(SerwerError::InvalidPathSlashes);
        }

        let mut segments: Vec<Segment> = vec![];
        let mut segments_length = 0;

        if segments_string != "/" {
            let parts: Vec<String> = segments_string[1..segments_string.len()]
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
            query_params,
        })
    }

    pub fn matches(&self, other_path: &Path) -> (bool, Option<Params>) {
        let mut params = Params::new();

        if (self.segments_length != other_path.segments_length)
            || (self.contains_params() && other_path.contains_params())
        {
            return (false, None);
        }

        for (segment, other_segment) in self.segments.iter().zip(&other_path.segments) {
            let mut is_param = false;

            if segment.is_param() {
                is_param = true;
                params.set_param(segment.get_string(), other_segment.get_string());
            }

            if other_segment.is_param() {
                is_param = true;
                params.set_param(other_segment.get_string(), segment.get_string());
            }

            if !is_param && segment.get_string() != other_segment.get_string() {
                return (false, None);
            }
        }

        (true, Some(params))
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }

    pub fn contains_params(&self) -> bool {
        let mut contains_params = false;

        for segment in self.segments.iter() {
            if segment.is_param() {
                contains_params = true;
            }
        }

        contains_params
    }

    pub fn get_query_params(&self) -> &QueryParams {
        &self.query_params
    }

    pub fn get_query_param(&self, key: &str) -> Option<String> {
        self.query_params.get_query_param(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::QueryParams;

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
                query_params: QueryParams::new(),
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
                query_params: QueryParams::new(),
            })
        );
    }

    #[test]
    fn test_from_string_with_query_params() {
        let string = &String::from("/user?id=1");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user?id=1"),
                segments: vec![Segment::from_string("user").unwrap()],
                segments_length: 1,
                query_params: QueryParams::from_string("id=1").unwrap(),
            })
        );

        let string = &String::from("/user?");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathSlashes));

        let string = &String::from("/");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/"),
                segments: vec![],
                segments_length: 0,
                query_params: QueryParams::new(),
            })
        );

        let string = &String::from("/?id=1");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/?id=1"),
                segments: vec![],
                segments_length: 0,
                query_params: QueryParams::from_string("id=1").unwrap(),
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

        let path = Path::from_string(&String::from("/user/<user_id>/post/2")).unwrap();
        let other_path = Path::from_string(&String::from("/user/1/post/<post_id>")).unwrap();
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
