use super::Path;
use crate::enums::{Method, SerwerError};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: Path,
    params: HashMap<String, String>,
    version: String,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, SerwerError> {
        let mut buf_reader = BufReader::new(&*stream);
        let buffer = &mut String::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        buf_reader.read_line(buffer).unwrap();
        let first_line: Vec<&str> = buffer.split(" ").collect();

        let method = match first_line[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => return Err(SerwerError::MethodNotFound),
        };

        let path = Path::from_string(String::from(first_line[1])).unwrap();

        let version = String::from(first_line[2].trim());
        buffer.clear();

        while let Ok(_) = buf_reader.read_line(buffer) {
            let trimmed_buffer = buffer.trim();
            if trimmed_buffer == "" {
                break;
            }

            let header_parts: Vec<&str> = trimmed_buffer.split(":").collect();
            headers.insert(
                header_parts[0].trim().to_string(),
                header_parts[1].trim().to_string(),
            );

            buffer.clear();
        }

        let cookie_header: String = headers
            .get("Cookie")
            .unwrap_or(&String::from(""))
            .parse()
            .unwrap();

        let mut cookies: HashMap<String, String> = HashMap::new();
        let cookies_parts: Vec<&str> = cookie_header.split("; ").collect();

        for cookie in cookies_parts.iter() {
            let cookie_parts: Vec<&str> = cookie.split("=").collect();
            cookies.insert(String::from(cookie_parts[0]), String::from(cookie_parts[1]));
        }

        let content_length: usize = headers
            .get("Content-Length")
            .unwrap_or(&String::from("0"))
            .parse()
            .unwrap();
        let mut body: Option<String> = None;

        if content_length > 0 {
            let mut body_buffer = vec![0; content_length];
            buf_reader.read_exact(&mut body_buffer).unwrap();
            body = Some(String::from_utf8(body_buffer).unwrap());
        }

        Ok(Self {
            method,
            path,
            params: HashMap::new(),
            version,
            headers,
            cookies,
            body,
        })
    }

    pub fn get_method(&self) -> Method {
        self.method.to_owned()
    }

    pub fn get_path(&self) -> Path {
        self.path.to_owned()
    }

    pub fn get_version(&self) -> String {
        self.version.to_owned()
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.to_owned()
    }

    pub fn get_body(&self) -> Option<String> {
        self.body.to_owned()
    }

    pub fn get_params(&self) -> HashMap<String, String> {
        self.params.to_owned()
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        match self.params.get(key) {
            Some(string) => Some(string.clone()),
            None => None.to_owned(),
        }
    }

    pub fn get_cookies(&self) -> HashMap<String, String> {
        self.cookies.to_owned()
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        match self.params.get(key) {
            Some(string) => Some(string.clone()),
            None => None.to_owned(),
        }
    }
}
