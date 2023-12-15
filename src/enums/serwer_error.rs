#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    InvalidPathSlashes,
    EmptyPathSegment,
    InvalidPathSegmentCharacters,
    MethodNotFound,
}
