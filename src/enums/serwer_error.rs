#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    InvalidPathSlashes,
    InvalidPathQueryParam,
    EmptyPathQueryParam,
    InvalidPathQueryParamCharacters,
    EmptyPathSegment,
    InvalidPathSegmentCharacters,
    InvalidCookie,
    EmptyCookie,
    InvalidCookieCharacters,
    MethodNotFound,
}
