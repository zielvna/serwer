use crate::{
    enums::{Method, SerwerError},
    utils::validate_path,
};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    version: String,
    headers: HashMap<String, String>,
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

        let path = String::from(&first_line[1][1..first_line[1].len()]);
        validate_path(&path)?;

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
            version,
            headers,
            body,
        })
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }
}
