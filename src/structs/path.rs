use super::{Params, QueryParams, Segment};
use crate::enums::SerwerError;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    string: String,
    segments: Vec<Segment>,
    query_params: QueryParams,
}

impl Path {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let parts: Vec<&str> = string.splitn(2, "#").collect();
        let parts: Vec<&str> = parts[0].splitn(2, "?").collect();

        let mut query_params = QueryParams::new();

        if parts.len() == 2 {
            let parts: Vec<&str> = parts[1].splitn(2, "#").collect();
            query_params = QueryParams::from_string(&parts[0])?;
        }

        let segments_string = parts[0];

        if !segments_string.starts_with("/") {
            return Err(SerwerError::PathShouldStartWithSlash(String::from(string)));
        }

        let mut segments: Vec<Segment> = vec![];

        let parts: Vec<&str> = segments_string[1..segments_string.len()]
            .split("/")
            .collect();

        for part in parts.iter() {
            let segment = Segment::from_string(part)?;
            segments.push(segment);
        }

        let mut params: Vec<&Segment> = vec![];

        for segment in segments.iter() {
            if segment.is_param() {
                if params.contains(&segment) {
                    return Err(SerwerError::PathContainsDuplicateParams);
                }

                params.push(segment);
            }
        }

        Ok(Self {
            string: String::from(string),
            segments,
            query_params,
        })
    }

    pub fn matches(&self, other_path: &Path) -> (bool, Option<Params>) {
        let mut params = Params::new();

        if (self.segments.len() != other_path.segments.len())
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

    pub fn get_query_param(&self, key: &str) -> Option<&String> {
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
                query_params: QueryParams::new(),
            })
        );

        let string = &String::from("/user/");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user/"),
                segments: vec![
                    Segment::from_string("user").unwrap(),
                    Segment::from_string("").unwrap()
                ],
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
                query_params: QueryParams::new(),
            })
        );

        let string = &String::from("/user//<id>");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user//<id>"),
                segments: vec![
                    Segment::from_string("user").unwrap(),
                    Segment::from_string("").unwrap(),
                    Segment::from_string("<id>").unwrap()
                ],
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
                query_params: QueryParams::from_string("id=1").unwrap(),
            })
        );

        let string = &String::from("/user?");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::InvalidPathQueryParam));
    }

    #[test]
    fn test_from_string_with_fragment() {
        let string = &String::from("/user#header");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user#header"),
                segments: vec![Segment::from_string("user").unwrap()],
                query_params: QueryParams::new(),
            })
        );

        let string = &String::from("/user?id=1#header");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/user?id=1#header"),
                segments: vec![Segment::from_string("user").unwrap()],
                query_params: QueryParams::from_string("id=1").unwrap(),
            })
        );
    }

    #[test]
    fn test_from_string_empty() {
        let string = &String::from("");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Err(SerwerError::PathShouldStartWithSlash(string.clone()))
        );

        let string = &String::from("/");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/"),
                segments: vec![Segment::from_string("").unwrap()],
                query_params: QueryParams::new(),
            })
        );

        let string = &String::from("/?id=1");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Ok(Path {
                string: String::from("/?id=1"),
                segments: vec![Segment::from_string("").unwrap()],
                query_params: QueryParams::from_string("id=1").unwrap(),
            })
        );
    }

    #[test]
    fn test_from_string_invalid_slashes() {
        let string = &String::from("user");
        let result = Path::from_string(string);
        assert_eq!(
            result,
            Err(SerwerError::PathShouldStartWithSlash(string.clone()))
        );
    }

    #[test]
    fn test_from_string_duplicate_params() {
        let string = &String::from("/user/<id>/<id>");
        let result = Path::from_string(string);
        assert_eq!(result, Err(SerwerError::PathContainsDuplicateParams));
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
    fn test_matches_empty() {
        let path = Path::from_string(&String::from("/")).unwrap();
        let other_path = Path::from_string(&String::from("/")).unwrap();
        let result = path.matches(&other_path);
        let params = Params::new();
        assert_eq!(result, (true, Some(params)));

        let path = Path::from_string(&String::from("/")).unwrap();
        let other_path = Path::from_string(&String::from("/<>")).unwrap();
        let result = path.matches(&other_path);
        let mut params = Params::new();
        params.set_param("", "");
        assert_eq!(result, (true, Some(params)));
    }

    #[test]
    fn test_matches_with_query_params_and_fragments() {
        let path = Path::from_string(&String::from("/user/3?show=true")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        let mut params = Params::new();
        params.set_param("id", "3");
        assert_eq!(result, (true, Some(params.clone())));

        let path = Path::from_string(&String::from("/user/3#header")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (true, Some(params.clone())));

        let path = Path::from_string(&String::from("/user/3?show=true#header")).unwrap();
        let other_path = Path::from_string(&String::from("/user/<id>")).unwrap();
        let result = path.matches(&other_path);
        assert_eq!(result, (true, Some(params)));
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
}
