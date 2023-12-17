#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    RequestBufferReadError,
    InvalidRequestStart,
    InvalidMethod,
    InvalidPathSlashes,
    InvalidPathQueryParam,
    InvalidPathQueryParamCharacters,
    EmptyPathSegment,
    PathQueryParamDecodeError,
    InvalidPathSegmentCharacters,
    InvalidVersion,
    InvalidHeader,
    InvalidHeaderCharacters,
    InvalidCookie,
    InvalidCookieCharacters,
    InvalidRequestBody,
}
