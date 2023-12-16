use super::{Cookies, Params, Path};
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
    params: Params,
    version: String,
    headers: HashMap<String, String>,
    cookies: Cookies,
    body: Option<String>,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, SerwerError> {
        let mut buf_reader = BufReader::new(&*stream);
        let buffer = &mut String::new();
        buf_reader.read_line(buffer).unwrap();

        let first_line: Vec<&str> = buffer.split(" ").collect();
        let method_string = first_line[0];
        let path_string = first_line[1];
        let version_string = first_line[2].trim();

        let method = Method::from_string(method_string)?;
        let path = Path::from_string(path_string)?;
        let version = String::from(version_string);

        buffer.clear();

        let mut headers: HashMap<String, String> = HashMap::new();

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

        let cookies_string = headers
            .get("Cookie")
            .unwrap_or(&String::from(""))
            .to_string();
        let cookies = Cookies::from_string(&cookies_string)?;

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
            params: Params::new(),
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

    pub fn get_params(&self) -> Params {
        self.params.to_owned()
    }

    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        self.params.get_param(key)
    }

    pub fn get_cookies(&self) -> Cookies {
        self.cookies.to_owned()
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        self.cookies.get_cookie(key)
    }

    pub fn get_query_param(&self, key: &str) -> Option<String> {
        self.path.get_query_param(key)
    }
}
