#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
}

impl std::string::ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::GET => String::from("GET"),
            Method::POST => String::from("POST"),
        }
    }
}
