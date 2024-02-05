use crate::SerwerError;

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    ALL,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Method {
    pub fn from_string(string: &str) -> Result<Self, SerwerError> {
        match string {
            "ALL" => Ok(Method::ALL),
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(SerwerError::InvalidMethod(String::from(string))),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::ALL => String::from("ALL"),
            Method::GET => String::from("GET"),
            Method::HEAD => String::from("HEAD"),
            Method::POST => String::from("POST"),
            Method::PUT => String::from("PUT"),
            Method::DELETE => String::from("OPTIONS"),
            Method::CONNECT => String::from("OPTIONS"),
            Method::OPTIONS => String::from("OPTIONS"),
            Method::TRACE => String::from("TRACE"),
            Method::PATCH => String::from("PATCH"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(Method::from_string("ALL").unwrap(), Method::ALL);
        assert_eq!(Method::from_string("GET").unwrap(), Method::GET);
        assert_eq!(Method::from_string("HEAD").unwrap(), Method::HEAD);
        assert_eq!(Method::from_string("POST").unwrap(), Method::POST);
        assert_eq!(Method::from_string("PUT").unwrap(), Method::PUT);
        assert_eq!(Method::from_string("DELETE").unwrap(), Method::DELETE);
        assert_eq!(Method::from_string("CONNECT").unwrap(), Method::CONNECT);
        assert_eq!(Method::from_string("OPTIONS").unwrap(), Method::OPTIONS);
        assert_eq!(Method::from_string("TRACE").unwrap(), Method::TRACE);
        assert_eq!(Method::from_string("PATCH").unwrap(), Method::PATCH);
        assert!(matches!(
            Method::from_string("INVALID"),
            Err(SerwerError::InvalidMethod(error_string)) if &error_string == "INVALID"
        ));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Method::ALL.to_string(), "ALL");
        assert_eq!(Method::GET.to_string(), "GET");
        assert_eq!(Method::HEAD.to_string(), "HEAD");
        assert_eq!(Method::POST.to_string(), "POST");
        assert_eq!(Method::PUT.to_string(), "PUT");
        assert_eq!(Method::DELETE.to_string(), "OPTIONS");
        assert_eq!(Method::CONNECT.to_string(), "OPTIONS");
        assert_eq!(Method::OPTIONS.to_string(), "OPTIONS");
        assert_eq!(Method::TRACE.to_string(), "TRACE");
        assert_eq!(Method::PATCH.to_string(), "PATCH");
    }
}
