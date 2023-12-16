#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    InvalidPathSlashes,
    InvalidPathQueryParam,
    EmptyPathQueryParam,
    InvalidPathQueryParamCharacters,
    EmptyPathSegment,
    InvalidPathSegmentCharacters,
    InvalidHeader,
    EmptyHeader,
    InvalidHeaderCharacters,
    InvalidCookie,
    EmptyCookie,
    InvalidCookieCharacters,
    MethodNotFound,
}
