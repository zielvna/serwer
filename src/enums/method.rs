use std::string::ToString;

use super::SerwerError;

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
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
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(SerwerError::MethodNotFound),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
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
