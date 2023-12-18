use super::{Cookies, Headers, Params, Path};
use crate::enums::{Method, SerwerError, Version};
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug, Clone, PartialEq)]
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
    pub(crate) fn from_stream(stream: &TcpStream) -> Result<Self, SerwerError> {
        let mut buf_reader = BufReader::new(stream);
        let buffer = &mut String::new();

        buf_reader
            .read_line(buffer)
            .map_err(|_| SerwerError::RequestBufferReadError)?;

        if !buffer.ends_with("\r\n") {
            return Err(SerwerError::InvalidRequestLine);
        }

        let parsed_buffer = buffer.trim_end_matches("\r\n");
        let first_line: Vec<&str> = parsed_buffer.split(" ").collect();

        if first_line.len() != 3 {
            return Err(SerwerError::InvalidRequestLine);
        }

        let (method_string, path_string, version_string) =
            (first_line[0], first_line[1], first_line[2]);

        let method = Method::from_string(method_string)?;
        let path = Path::from_string(path_string)?;
        let version = Version::from_string(version_string)?;

        let mut headers = Headers::new();

        loop {
            buffer.clear();

            buf_reader
                .read_line(buffer)
                .map_err(|_| SerwerError::RequestBufferReadError)?;

            if !buffer.ends_with("\r\n") {
                return Err(SerwerError::InvalidRequestHeaders);
            }

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

    pub fn get_original_url(&self) -> String {
        self.path.get_string().to_owned()
    }

    pub fn get_query_param(&self, key: &str) -> Option<String> {
        self.path.get_query_param(key).cloned()
    }

    pub fn get_version(&self) -> Version {
        self.version.to_owned()
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get_header(key).cloned()
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        self.cookies.get_cookie(key).cloned()
    }

    pub fn get_body(&self) -> Option<String> {
        self.body.to_owned()
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        self.params.get_param(key).cloned()
    }

    pub(crate) fn get_path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn set_params(&mut self, params: Params) {
        self.params = params;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, net::TcpListener, thread};

    fn request_from_bytes(data: &[u8]) -> Result<Request, SerwerError> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();

        let buf = data.to_owned();

        thread::spawn(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            stream.write_all(&buf).unwrap();
        });

        let (stream, _) = listener.accept().unwrap();
        Request::from_stream(&stream)
    }

    #[test]
    fn test_from_stream() {
        let result = request_from_bytes("GET / HTTP/1.1\r\n\r\n".as_bytes());

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), Method::GET);
        assert_eq!(result.get_original_url(), "/");
        assert_eq!(result.get_version(), Version::HTTP_1_1);
        assert_eq!(result.get_body(), None);
    }

    #[test]
    fn test_from_stream_headers() {
        let result = request_from_bytes(
            "GET / HTTP/1.1\r\nHost: localhost:80\r\nConnection: keep-alive\r\n\r\n".as_bytes(),
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), Method::GET);
        assert_eq!(result.get_original_url(), "/");
        assert_eq!(result.get_version(), Version::HTTP_1_1);
        assert_eq!(result.get_body(), None);
        assert_eq!(
            result.get_header("Host"),
            Some(String::from("localhost:80"))
        );
        assert_eq!(
            result.get_header("Connection"),
            Some(String::from("keep-alive"))
        );
    }

    #[test]
    fn test_from_stream_cookies() {
        let result =
            request_from_bytes("GET / HTTP/1.1\r\nCookie: id=1; name=John\r\n\r\n".as_bytes());

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), Method::GET);
        assert_eq!(result.get_original_url(), "/");
        assert_eq!(result.get_version(), Version::HTTP_1_1);
        assert_eq!(result.get_body(), None);
        assert_eq!(result.get_cookie("id"), Some(String::from("1")));
        assert_eq!(result.get_cookie("name"), Some(String::from("John")));
    }

    #[test]
    fn test_from_stream_body() {
        let result = request_from_bytes(
            "POST / HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World".as_bytes(),
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), Method::POST);
        assert_eq!(result.get_original_url(), "/");
        assert_eq!(result.get_version(), Version::HTTP_1_1);
        assert_eq!(result.get_body(), Some(String::from("Hello World")));
    }

    #[test]
    fn test_from_stream_query_params() {
        let result = request_from_bytes("GET /?id=1&name=John HTTP/1.1\r\n\r\n".as_bytes());

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_method(), Method::GET);
        assert_eq!(result.get_original_url(), "/?id=1&name=John");
        assert_eq!(result.get_version(), Version::HTTP_1_1);
        assert_eq!(result.get_body(), None);
        assert_eq!(result.get_query_param("id"), Some(String::from("1")));
        assert_eq!(result.get_query_param("name"), Some(String::from("John")));
    }

    #[test]
    fn test_from_stream_invalid_request_line() {
        let result = request_from_bytes("GET / HTTP/1.1".as_bytes());
        assert_eq!(result, Err(SerwerError::InvalidRequestLine));

        let result = request_from_bytes("GET /".as_bytes());
        assert_eq!(result, Err(SerwerError::InvalidRequestLine));
    }

    #[test]
    fn test_from_stream_invalid_request_headers() {
        let result = request_from_bytes("GET / HTTP/1.1\r\nHost: localhost:80".as_bytes());

        assert_eq!(result, Err(SerwerError::InvalidRequestHeaders));
    }

    #[test]
    fn test_from_stream_invalid_request_body() {
        let mut bytes = "POST / HTTP/1.1\r\nContent-Length: 12\r\n\r\nHello World"
            .as_bytes()
            .to_vec();
        bytes.push(128);
        let result = request_from_bytes(bytes.as_slice());

        assert_eq!(result, Err(SerwerError::InvalidRequestBody));
    }
}
