#[derive(Debug, PartialEq)]
pub enum SerwerError {
    InvalidPathCharacters,
    InvalidPathSlashes,
    InvalidPathPart,
    MethodNotFound,
}
