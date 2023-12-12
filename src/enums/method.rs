use std::string::ToString;

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
