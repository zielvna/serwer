use super::Segment;
use crate::enums::SerwerError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Path {
    string: String,
    segments: Vec<Segment>,
    segments_length: usize,
}

impl Path {
    pub fn from_string(string: &String) -> Result<Self, SerwerError> {
        let mut string = string.clone();

        if !string.starts_with("/") || string.ends_with("/") || string.contains("//") {
            return Err(SerwerError::InvalidPathSlashes);
        }

        string.remove(0);

        let parts: Vec<String> = string.split("/").map(String::from).collect();
        let mut segments: Vec<Segment> = vec![];
        let mut segments_length = 0;

        for part in parts.iter() {
            let segment = Segment::from_string(part)?;
            segments.push(segment);
            segments_length += 1;
        }

        Ok(Self {
            string: String::from(string),
            segments,
            segments_length,
        })
    }

    pub fn matches(&self, other_path: &Path) -> (bool, Option<HashMap<String, String>>) {
        let mut params: HashMap<String, String> = HashMap::new();

        if self.segments_length != other_path.segments_length {
            return (false, None);
        }

        for i in 0..self.segments_length {
            let mut is_param = false;

            if self.segments[i].is_param() {
                is_param = true;
                params.insert(
                    self.segments[i].get_string().clone(),
                    other_path.segments[i].get_string().clone(),
                );
            }

            if other_path.segments[i].is_param() {
                is_param = true;
                params.insert(
                    other_path.segments[i].get_string().clone(),
                    self.segments[i].get_string().clone(),
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
