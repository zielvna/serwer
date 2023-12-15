use super::{Params, Segment};
use crate::enums::SerwerError;

#[derive(Debug, Clone)]
pub struct Path {
    string: String,
    segments: Vec<Segment>,
    segments_length: usize,
}

impl Path {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        let string = string.to_string();

        if !string.starts_with("/") || string.ends_with("/") || string.contains("//") {
            return Err(SerwerError::InvalidPathSlashes);
        }

        let parts: Vec<String> = string[1..string.len()]
            .split("/")
            .map(String::from)
            .collect();
        let mut segments: Vec<Segment> = vec![];
        let mut segments_length = 0;

        for part in parts.iter() {
            let segment = Segment::from_string(part)?;
            segments.push(segment);
            segments_length += 1;
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
