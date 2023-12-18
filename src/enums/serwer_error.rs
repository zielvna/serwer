#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    RequestBufferReadError,
    InvalidRequestStart,
    InvalidMethod,
    PathShouldStartWithSlash,
    InvalidPathQueryParam,
    InvalidPathQueryParamCharacters,
    PathQueryParamDecodeError,
    PathSegmentDecodeError,
    InvalidPathSegmentCharacters,
    PathContainsDuplicateParams,
    InvalidVersion,
    InvalidHeader,
    InvalidHeaderCharacters,
    InvalidCookie,
    InvalidCookieCharacters,
    InvalidRequestBody,
}
