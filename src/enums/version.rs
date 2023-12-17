use super::SerwerError;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Version {
    HTTP_0_9,
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2,
    HTTP_3,
}

impl Version {
    pub fn from_string(version_string: &str) -> Result<Self, SerwerError> {
        match version_string {
            "HTTP/0.9" => Ok(Version::HTTP_0_9),
            "HTTP/1.0" => Ok(Version::HTTP_1_0),
            "HTTP/1.1" => Ok(Version::HTTP_1_1),
            "HTTP/2" => Ok(Version::HTTP_2),
            "HTTP/3" => Ok(Version::HTTP_3),
            _ => Err(SerwerError::InvalidVersion),
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::HTTP_0_9 => String::from("HTTP/0.9"),
            Version::HTTP_1_0 => String::from("HTTP/1.0"),
            Version::HTTP_1_1 => String::from("HTTP/1.1"),
            Version::HTTP_2 => String::from("HTTP/2"),
            Version::HTTP_3 => String::from("HTTP/3"),
        }
    }
}
