#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    InvalidPathSlashes,
    InvalidPathQueryParams,
    InvalidPathQueryParam,
    InvalidPathQueryParamCharacters,
    EmptyPathSegment,
    InvalidPathSegmentCharacters,
    MethodNotFound,
}
