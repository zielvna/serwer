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
                write!(f, r#"Header missing tailing CRLF: {}"#, header)
            }
            SerwerError::InvalidHeader(header) => write!(f, r#"Invalid header: "{}"#, header),
            SerwerError::InvalidHeaderCharacters(header) => {
                write!(f, r#"Invalid header characters: "{}""#, header)
            }
            SerwerError::InvalidCookie(cookie) => write!(f, r#"Invalid cookie: "{}"#, cookie),
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
