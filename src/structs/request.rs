use super::{Cookies, Headers, Params, Path, QueryParams};
use crate::enums::{Method, SerwerError, Version};
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: Path,
    version: Version,
    headers: Headers,
    cookies: Cookies,
    body: Option<String>,
    params: Params,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, SerwerError> {
        let mut buf_reader = BufReader::new(stream);
        let buffer = &mut String::new();

        buf_reader
            .read_line(buffer)
            .map_err(|_| SerwerError::RequestBufferReadError)?;

        let parsed_buffer = buffer.trim_end_matches("\r\n");
        let first_line: Vec<&str> = parsed_buffer.split(" ").collect();

        if first_line.len() != 3 {
            return Err(SerwerError::InvalidRequestStart);
        }

        let method_string = first_line[0];
        let path_string = first_line[1];
        let version_string = first_line[2];

        let method = Method::from_string(method_string)?;
        let path = Path::from_string(path_string)?;
        let version = Version::from_string(version_string)?;

        let mut headers = Headers::new();

        loop {
            buffer.clear();

            buf_reader
                .read_line(buffer)
                .map_err(|_| SerwerError::RequestBufferReadError)?;

            let parsed_buffer = buffer.trim_end_matches("\r\n");

            if parsed_buffer.is_empty() {
                break;
            }

            headers.set_header_from_string(&parsed_buffer)?;
        }

        let cookies_string = headers
            .get_header("Cookie")
            .map(|s| s.to_owned())
            .unwrap_or_default();
        let cookies = Cookies::from_string(&cookies_string)?;

        let content_length: usize = headers
            .get_header("Content-Length")
            .unwrap_or(&String::from("0"))
            .parse()
            .unwrap_or_default();
        let mut body: Option<String> = None;

        if content_length > 0 {
            let mut body_buffer = vec![0; content_length];
            buf_reader
                .read_exact(&mut body_buffer)
                .map_err(|_| SerwerError::RequestBufferReadError)?;
            body =
                Some(String::from_utf8(body_buffer).map_err(|_| SerwerError::InvalidRequestBody)?);
        }

        Ok(Self {
            method,
            path,
            version,
            headers,
            cookies,
            body,
            params: Params::new(),
        })
    }

    pub fn get_method(&self) -> Method {
        self.method.to_owned()
    }

    pub fn get_path(&self) -> Path {
        self.path.to_owned()
    }

    pub fn get_query_param(&self, key: &str) -> Option<String> {
        self.path.get_query_param(key)
    }

    pub fn get_query_params(&self) -> QueryParams {
        self.path.get_query_params().to_owned()
    }

    pub fn get_version(&self) -> Version {
        self.version.to_owned()
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get_header(key).cloned()
    }

    pub fn get_headers(&self) -> Headers {
        self.headers.to_owned()
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        self.cookies.get_cookie(key).cloned()
    }

    pub fn get_cookies(&self) -> Cookies {
        self.cookies.to_owned()
    }

    pub fn get_body(&self) -> Option<String> {
        self.body.to_owned()
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        self.params.get_param(key)
    }

    pub fn get_params(&self) -> Params {
        self.params.to_owned()
    }

    pub(crate) fn set_params(&mut self, params: Params) {
        self.params = params;
    }
}
