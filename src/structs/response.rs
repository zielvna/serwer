use super::{cookie::Cookie, cookies::Cookies, headers::Headers};
use crate::{enums::StatusCode, Version};

#[derive(Debug, Clone)]
pub struct Response {
    version: Version,
    status_code: StatusCode,
    body: Vec<u8>,
    headers: Headers,
    cookies: Cookies,
}

impl Response {
    pub fn new(version: &Version) -> Self {
        Self {
            version: version.clone(),
            status_code: StatusCode::OK,
            body: vec![],
            headers: Headers::new(),
            cookies: Cookies::new(),
        }
    }

    pub fn set(&mut self, status_code: StatusCode, body: String) -> &mut Self {
        self.status_code = status_code;
        self.set_body(&body);
        self
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn set_header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.set_header(name, value);
        self
    }

    pub fn set_cookie(&mut self, name: &str, cookie: Cookie) -> &mut Self {
        self.cookies.set_cookie(name, cookie);
        self
    }

    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.set_header("Content-Length", body.len().to_string().as_str());
        self.body = body.as_bytes().to_vec();
        self
    }

    pub fn set_body_from_bytes(&mut self, body: Vec<u8>) -> &mut Self {
        self.set_header("Content-Length", body.len().to_string().as_str());
        self.body = body;
        self
    }

    pub(crate) fn write(self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];

        response.extend(self.version.to_string().as_bytes());
        response.extend(b" ");
        response.extend(self.status_code.to_string().as_bytes());
        response.extend(b"\r\n");

        response.extend(self.headers.to_bytes());

        response.extend(self.cookies.to_bytes());

        response.extend(b"\r\n");
        response.extend(self.body);

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut response = Response::new(&Version::HTTP_1_1);
        response.set(StatusCode::OK, "Hello World".to_string());
        let result = String::from_utf8(response.write()).unwrap();
        assert_eq!(
            result,
            "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World"
        );
    }

    #[test]
    fn test_write_with_status_code() {
        let mut response = Response::new(&Version::HTTP_1_1);
        response.set_status_code(StatusCode::NotFound);
        let result = String::from_utf8(response.write()).unwrap();
        assert_eq!(result, "HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn test_write_with_header() {
        let mut response = Response::new(&Version::HTTP_1_1);
        response.set_header("Content-Type", "text/html");
        let result = String::from_utf8(response.write()).unwrap();
        assert_eq!(result, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");
    }

    #[test]
    fn test_write_with_cookies() {
        let mut response = Response::new(&Version::HTTP_1_1);
        response.set_cookie("id", Cookie::new("id", "1"));
        let result = String::from_utf8(response.write()).unwrap();
        assert_eq!(result, "HTTP/1.1 200 OK\r\nSet-Cookie: id=1\r\n\r\n");
    }

    #[test]
    fn test_write_with_body() {
        let mut response = Response::new(&Version::HTTP_1_1);
        response.set_body("Hello World");
        let result = String::from_utf8(response.write()).unwrap();
        assert_eq!(
            result,
            "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World"
        );
    }
}
