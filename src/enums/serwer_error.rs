#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    RequestBufferReadError,
    InvalidRequestStart,
    InvalidMethod,
    InvalidPathSlashes,
    InvalidPathQueryParam,
    EmptyPathQueryParam,
    InvalidPathQueryParamCharacters,
    EmptyPathSegment,
    InvalidPathSegmentCharacters,
    InvalidVersion,
    InvalidHeader,
    InvalidHeaderCharacters,
    InvalidCookie,
    InvalidCookieCharacters,
    InvalidRequestBody,
}
