use crate::{Cookie, Cookies, Headers, Method, Params, Path, SerwerError, Version};
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
    body: Vec<u8>,
    params: Params,
}

impl Request {
    pub(crate) fn from_stream(stream: &TcpStream) -> Result<Self, SerwerError> {
        let mut buf_reader = BufReader::new(stream);
        let buffer = &mut String::new();

        buf_reader.read_line(buffer)?;

        if !buffer.ends_with("\r\n") {
            return Err(SerwerError::InvalidRequestLine(buffer.clone()));
        }

        let parsed_buffer = buffer.trim_end_matches("\r\n");
        let first_line: Vec<&str> = parsed_buffer.split(" ").collect();

        if first_line.len() != 2 && first_line.len() != 3 {
            return Err(SerwerError::InvalidRequestLine(String::from(parsed_buffer)));
        }

        let (method_string, path_string) = (first_line[0], first_line[1]);

        let method = Method::from_string(method_string)?;
        let path = Path::from_string(path_string)?;

        let version = if first_line.len() == 3 {
            let version_string = first_line[2];
            Version::from_string(version_string)?
        } else {
            Version::HTTP_0_9
        };

        let mut headers = Headers::new();

        loop {
            buffer.clear();

            buf_reader.read_line(buffer)?;

            if !buffer.ends_with("\r\n") {
                return Err(SerwerError::HeaderMissingTailingCRLF(buffer.clone()));
            }

            let parsed_buffer = buffer.trim_end_matches("\r\n");

            if parsed_buffer.is_empty() {
                break;
            }

            headers.set_header_from_string(&parsed_buffer)?;
        }

        let cookies_string = headers
            .header("cookie")
            .map(|s| s.to_owned())
            .unwrap_or_default();
        let cookies = Cookies::from_string(&cookies_string)?;

        let content_length: usize = headers
            .header("content-length")
            .unwrap_or(&String::from("0"))
            .parse()
            .unwrap_or_default();
        let mut body: Vec<u8> = vec![];

        if content_length > 0 {
            let mut body_buffer = vec![0; content_length];
            buf_reader.read_exact(&mut body_buffer)?;
            body = body_buffer.to_vec();
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

    pub fn method(&self) -> Method {
        self.method.to_owned()
    }

    pub fn original_url(&self) -> String {
        self.path.original_url().to_owned()
    }

    pub fn query_param(&self, key: &str) -> Option<String> {
        self.path.query_param(key).cloned()
    }

    pub fn version(&self) -> Version {
        self.version.to_owned()
    }

    pub fn header(&self, key: &str) -> Option<String> {
        self.headers.header(key).cloned()
    }

    pub fn cookie(&self, key: &str) -> Option<Cookie> {
        self.cookies.cookie(key).cloned()
    }

    pub fn body(&self) -> Result<String, SerwerError> {
        let string = String::from_utf8(self.body.clone())?;
        Ok(string)
    }

    pub fn body_as_bytes(&self) -> Vec<u8> {
        self.body.to_owned()
    }

    pub fn param(&self, key: &str) -> Option<String> {
        self.params.param(key).cloned()
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn set_params(&mut self, params: Params) {
        self.params = params;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request_from_bytes;

    #[test]
    fn test_from_stream() {
        let result = request_from_bytes("GET / HTTP/1.1\r\n\r\n".as_bytes()).unwrap();

        assert_eq!(result.method(), Method::GET);
        assert_eq!(result.original_url(), "/");
        assert_eq!(result.version(), Version::HTTP_1_1);
        assert_eq!(result.body().unwrap(), String::from(""));
    }

    #[test]
    fn test_from_stream_http_0_9() {
        let result = request_from_bytes("GET /\r\n\r\n".as_bytes()).unwrap();

        assert_eq!(result.method(), Method::GET);
        assert_eq!(result.original_url(), "/");
        assert_eq!(result.version(), Version::HTTP_0_9);
        assert_eq!(result.body().unwrap(), String::from(""));
    }

    #[test]
    fn test_from_stream_headers() {
        let result = request_from_bytes(
            "GET / HTTP/1.1\r\nHost: localhost:80\r\nConnection: keep-alive\r\n\r\n".as_bytes(),
        )
        .unwrap();

        assert_eq!(result.method(), Method::GET);
        assert_eq!(result.original_url(), "/");
        assert_eq!(result.version(), Version::HTTP_1_1);
        assert_eq!(result.body().unwrap(), String::from(""));
        assert_eq!(result.header("Host"), Some(String::from("localhost:80")));
        assert_eq!(
            result.header("Connection"),
            Some(String::from("keep-alive"))
        );
    }

    #[test]
    fn test_from_stream_cookies() {
        let result =
            request_from_bytes("GET / HTTP/1.1\r\nCookie: id=1; name=John\r\n\r\n".as_bytes())
                .unwrap();

        assert_eq!(result.method(), Method::GET);
        assert_eq!(result.original_url(), "/");
        assert_eq!(result.version(), Version::HTTP_1_1);
        assert_eq!(result.body().unwrap(), String::from(""));
        assert_eq!(
            result.cookie("id"),
            Some(Cookie::from_string("id=1").unwrap())
        );
        assert_eq!(
            result.cookie("name"),
            Some(Cookie::from_string("name=John").unwrap())
        );
    }

    #[test]
    fn test_from_stream_body() {
        let result = request_from_bytes(
            "POST / HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World".as_bytes(),
        )
        .unwrap();

        assert_eq!(result.method(), Method::POST);
        assert_eq!(result.original_url(), "/");
        assert_eq!(result.version(), Version::HTTP_1_1);
        assert_eq!(result.body().unwrap(), String::from("Hello World"));
        assert_eq!(
            result.body_as_bytes(),
            String::from("Hello World").as_bytes()
        );
    }

    #[test]
    fn test_from_stream_query_params() {
        let result =
            request_from_bytes("GET /?id=1&name=John HTTP/1.1\r\n\r\n".as_bytes()).unwrap();

        assert_eq!(result.method(), Method::GET);
        assert_eq!(result.original_url(), "/?id=1&name=John");
        assert_eq!(result.version(), Version::HTTP_1_1);
        assert_eq!(result.body().unwrap(), String::from(""));
        assert_eq!(result.query_param("id"), Some(String::from("1")));
        assert_eq!(result.query_param("name"), Some(String::from("John")));
    }

    #[test]
    fn test_from_stream_path() {
        let result = request_from_bytes("GET / HTTP/1.1\r\n\r\n".as_bytes()).unwrap();

        assert_eq!(result.path(), &Path::from_string("/").unwrap());
    }

    #[test]
    fn test_from_stream_set_params() {
        let mut result = request_from_bytes("GET / HTTP/1.1\r\n\r\n".as_bytes()).unwrap();
        let mut params = Params::new();
        params.set_param("user", "1");
        result.set_params(params);

        assert_eq!(result.param("user").unwrap(), String::from("1"));
    }

    #[test]
    fn test_from_stream_invalid_request_line() {
        let result = request_from_bytes("GET / HTTP/1.1".as_bytes());

        assert!(matches!(
            result,
            Err(SerwerError::InvalidRequestLine(error_string)) if &error_string == "GET / HTTP/1.1"
        ));

        let result = request_from_bytes("GET\r\n\r\n".as_bytes());
        assert!(matches!(
            result,
            Err(SerwerError::InvalidRequestLine(error_string)) if &error_string == "GET"
        ));
    }

    #[test]
    fn test_from_stream_invalid_request_headers() {
        let result = request_from_bytes("GET / HTTP/1.1\r\nHost: localhost:80".as_bytes());

        assert!(matches!(
            result,
            Err(SerwerError::HeaderMissingTailingCRLF(error_string)) if &error_string == "Host: localhost:80"
        ));
    }

    #[test]
    fn test_from_stream_body_not_utf8() {
        let mut bytes = "POST / HTTP/1.1\r\nContent-Length: 12\r\n\r\nHello World"
            .as_bytes()
            .to_vec();
        bytes.push(128);
        let result = request_from_bytes(bytes.as_slice()).unwrap();

        assert!(matches!(result.body(), Err(SerwerError::FromUtf8Error(_))));
    }
}
