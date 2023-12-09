#[derive(Clone)]
pub enum StatusCode {
    OK = 200,
    NotFound = 404,
}

impl std::string::ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::OK => String::from("200 OK"),
            StatusCode::NotFound => String::from("404 Not Found"),
        }
    }
}
