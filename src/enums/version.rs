use crate::SerwerError;

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
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        match string {
            "HTTP/0.9" => Ok(Version::HTTP_0_9),
            "HTTP/1.0" => Ok(Version::HTTP_1_0),
            "HTTP/1.1" => Ok(Version::HTTP_1_1),
            "HTTP/2" => Ok(Version::HTTP_2),
            "HTTP/3" => Ok(Version::HTTP_3),
            _ => Err(SerwerError::InvalidVersion(String::from(string))),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(Version::from_string("HTTP/0.9").unwrap(), Version::HTTP_0_9);
        assert_eq!(Version::from_string("HTTP/1.0").unwrap(), Version::HTTP_1_0);
        assert_eq!(Version::from_string("HTTP/1.1").unwrap(), Version::HTTP_1_1);
        assert_eq!(Version::from_string("HTTP/2").unwrap(), Version::HTTP_2);
        assert_eq!(Version::from_string("HTTP/3").unwrap(), Version::HTTP_3);
        assert!(matches!(
            Version::from_string("HTTP/1.2"),
            Err(SerwerError::InvalidVersion(error_string)) if &error_string == "HTTP/1.2"
        ));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Version::HTTP_0_9.to_string(), "HTTP/0.9");
        assert_eq!(Version::HTTP_1_0.to_string(), "HTTP/1.0");
        assert_eq!(Version::HTTP_1_1.to_string(), "HTTP/1.1");
        assert_eq!(Version::HTTP_2.to_string(), "HTTP/2");
        assert_eq!(Version::HTTP_3.to_string(), "HTTP/3");
    }
}
