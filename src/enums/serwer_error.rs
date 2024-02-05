use std::{fmt, io, num, string};

#[derive(Debug)]
pub enum SerwerError {
    RequestBufferReadError,
    InvalidRequestLine(String),
    InvalidMethod(String),
    PathMissingLeadingSlash(String),
    InvalidQueryParam(String),
    InvalidQueryParamCharacters(String),
    InvalidPathSegmentCharacters(String),
    PathContainsDuplicateParams(String),
    InvalidVersion(String),
    HeaderMissingTailingCRLF(String),
    InvalidHeader(String),
    InvalidHeaderCharacters(String),
    InvalidCookie(String),
    InvalidCookieCharacters(String),
    DecodeError(String),
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    FromUtf8Error(string::FromUtf8Error),
}

impl From<io::Error> for SerwerError {
    fn from(error: io::Error) -> Self {
        SerwerError::IoError(error)
    }
}

impl From<num::ParseIntError> for SerwerError {
    fn from(error: num::ParseIntError) -> Self {
        SerwerError::ParseIntError(error)
    }
}

impl From<string::FromUtf8Error> for SerwerError {
    fn from(error: string::FromUtf8Error) -> Self {
        SerwerError::FromUtf8Error(error)
    }
}

impl fmt::Display for SerwerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SerwerError::RequestBufferReadError => {
                write!(f, r#"Error while reading request buffer"#)
            }
            SerwerError::InvalidRequestLine(request_line) => {
                write!(f, r#"Invalid request line: "{}""#, request_line)
            }
            SerwerError::InvalidMethod(method) => write!(f, r#"Invalid method: "{}""#, method),
            SerwerError::PathMissingLeadingSlash(path) => {
                write!(f, r#"Path missing leading slash: "{}""#, path)
            }
            SerwerError::InvalidQueryParam(query_param) => {
                write!(f, r#"Invalid query param: "{}""#, query_param)
            }
            SerwerError::InvalidQueryParamCharacters(query_param) => {
                write!(f, r#"Invalid query param characters: "{}""#, query_param)
            }
            SerwerError::InvalidPathSegmentCharacters(path_segment) => {
                write!(f, r#"Invalid path segment characters: "{}""#, path_segment)
            }
            SerwerError::PathContainsDuplicateParams(path) => {
                write!(f, r#"Path contains duplicate params: "{}""#, path)
            }
            SerwerError::InvalidVersion(version) => {
                write!(f, r#"Invalid version: "{}""#, version)
            }
            SerwerError::HeaderMissingTailingCRLF(header) => {
                write!(f, r#"Header missing tailing CRLF: "{}""#, header)
            }
            SerwerError::InvalidHeader(header) => write!(f, r#"Invalid header: "{}""#, header),
            SerwerError::InvalidHeaderCharacters(header) => {
                write!(f, r#"Invalid header characters: "{}""#, header)
            }
            SerwerError::InvalidCookie(cookie) => write!(f, r#"Invalid cookie: "{}""#, cookie),
            SerwerError::InvalidCookieCharacters(cookie) => {
                write!(f, r#"Invalid cookie characters: "{}""#, cookie)
            }
            SerwerError::DecodeError(string) => write!(f, r#"Decode error: "{}""#, string),
            SerwerError::IoError(error) => write!(f, "IO error: {}", error),
            SerwerError::ParseIntError(error) => write!(f, "Parse int error: {}", error),
            SerwerError::FromUtf8Error(error) => write!(f, "From utf8 error: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_from_bytes;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_from() {
        let mut buffer = BufReader::new(stream_from_bytes(&[255]));
        let string = &mut String::from("");

        assert!(matches!(
            SerwerError::from(buffer.read_line(string).unwrap_err()),
            SerwerError::IoError(error) if error.to_string() == "stream did not contain valid UTF-8"
        ));

        assert!(matches!(
            SerwerError::from(u8::from_str_radix(&"a", 10).unwrap_err()),
            SerwerError::ParseIntError(error) if error.to_string() == "invalid digit found in string"
        ));

        assert!(matches!(
            SerwerError::from(String::from_utf8(vec![255]).unwrap_err()),
            SerwerError::FromUtf8Error(error) if error.to_string() == "invalid utf-8 sequence of 1 bytes from index 0"
        ));
    }

    #[test]
    fn test_display() {
        assert_eq!(
            SerwerError::RequestBufferReadError.to_string(),
            "Error while reading request buffer"
        );

        assert_eq!(
            SerwerError::InvalidRequestLine(String::from("GET / HTTP/1.1")).to_string(),
            r#"Invalid request line: "GET / HTTP/1.1""#
        );

        assert_eq!(
            SerwerError::InvalidMethod(String::from("GET")).to_string(),
            r#"Invalid method: "GET""#
        );

        assert_eq!(
            SerwerError::PathMissingLeadingSlash(String::from("path")).to_string(),
            r#"Path missing leading slash: "path""#
        );

        assert_eq!(
            SerwerError::InvalidQueryParam(String::from("query_param")).to_string(),
            r#"Invalid query param: "query_param""#
        );

        assert_eq!(
            SerwerError::InvalidQueryParamCharacters(String::from("query_param")).to_string(),
            r#"Invalid query param characters: "query_param""#
        );

        assert_eq!(
            SerwerError::InvalidPathSegmentCharacters(String::from("path_segment")).to_string(),
            r#"Invalid path segment characters: "path_segment""#
        );

        assert_eq!(
            SerwerError::PathContainsDuplicateParams(String::from("path")).to_string(),
            r#"Path contains duplicate params: "path""#
        );

        assert_eq!(
            SerwerError::InvalidVersion(String::from("version")).to_string(),
            r#"Invalid version: "version""#
        );

        assert_eq!(
            SerwerError::HeaderMissingTailingCRLF(String::from("header")).to_string(),
            r#"Header missing tailing CRLF: "header""#
        );

        assert_eq!(
            SerwerError::InvalidHeader(String::from("header")).to_string(),
            r#"Invalid header: "header""#
        );

        assert_eq!(
            SerwerError::InvalidHeaderCharacters(String::from("header")).to_string(),
            r#"Invalid header characters: "header""#
        );

        assert_eq!(
            SerwerError::InvalidCookie(String::from("cookie")).to_string(),
            r#"Invalid cookie: "cookie""#
        );

        assert_eq!(
            SerwerError::InvalidCookieCharacters(String::from("cookie")).to_string(),
            r#"Invalid cookie characters: "cookie""#
        );

        assert_eq!(
            SerwerError::DecodeError(String::from("string")).to_string(),
            r#"Decode error: "string""#
        );

        let mut buffer = BufReader::new(stream_from_bytes(&[255]));
        let string = &mut String::from("");

        assert_eq!(
            SerwerError::IoError(buffer.read_line(string).unwrap_err()).to_string(),
            "IO error: stream did not contain valid UTF-8"
        );

        assert_eq!(
            SerwerError::ParseIntError(u8::from_str_radix(&"a", 10).unwrap_err()).to_string(),
            "Parse int error: invalid digit found in string"
        );

        assert_eq!(
            SerwerError::FromUtf8Error(String::from_utf8(vec![255]).unwrap_err()).to_string(),
            "From utf8 error: invalid utf-8 sequence of 1 bytes from index 0"
        );
    }
}
