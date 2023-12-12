#[derive(Debug, PartialEq, Clone)]
pub enum SerwerError {
    InvalidPathCharacters,
    InvalidPathSlashes,
    InvalidPathPart,
    MethodNotFound,
}
