use std::collections::HashMap;

use crate::enums::StatusCode;

#[derive(Debug, Clone)]
pub struct Response {
    version: String,
    status_code: StatusCode,
    body: Vec<u8>,
    content_length: usize,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
}

impl Response {
    pub fn default() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status_code: StatusCode::OK,
            body: vec![],
            content_length: 0,
            headers: HashMap::new(),
            cookies: HashMap::new(),
        }
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    pub fn set_version(&mut self, version: String) -> Self {
        self.version = version;

        self.clone()
    }

    pub fn get(&self) -> (StatusCode, Vec<u8>) {
        (self.status_code.clone(), self.body.clone())
    }

    pub fn set(&mut self, status_code: StatusCode, body: String) -> Self {
        self.status_code = status_code;
        self.content_length = body.len();
        self.body = body.as_bytes().to_vec();

        self.clone()
    }

    pub fn get_status_code(&self) -> StatusCode {
        self.status_code.clone()
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;

        self.clone()
    }

    pub fn get_body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: Vec<u8>) -> Self {
        self.content_length = body.len();
        self.body = body;

        self.clone()
    }

    pub fn set_body_from_string(&mut self, body: String) -> Self {
        self.content_length = body.len();
        self.body = body.as_bytes().to_vec();

        self.clone()
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> Self {
        self.headers.insert(String::from(key), String::from(value));

        self.clone()
    }

    pub fn set_cookie(&mut self, key: &str, value: &str) -> Self {
        self.cookies.insert(String::from(key), String::from(value));

        self.clone()
    }

    pub fn write(self) -> Vec<u8> {
        let mut response_string = vec![];

        let version = &self.version;
        let status_code = &self.status_code.to_string();
        response_string.extend(format!("{version} {status_code}\r\n").into_bytes());

        for (key, value) in self.headers.into_iter() {
            response_string.extend(format!("{key}: {value}\r\n").as_bytes());
        }

        if self.cookies.len() > 0 {
            response_string.extend("Set-Cookie: ".as_bytes());

            for (key, value) in self.cookies.into_iter() {
                response_string.extend(format!("{key}={value};").as_bytes());
            }

            response_string.extend("\r\n".as_bytes());
        }

        if self.content_length > 0 {
            let content_length = &self.content_length;
            response_string.extend(format!("Content-Length: {content_length}\r\n").as_bytes());
        }

        response_string.extend("\r\n".as_bytes());
        response_string.extend(&self.body);

        response_string
    }
}
