use super::{Action, Path, Request, Response};
use crate::enums::{Method, SerwerError};

#[derive(Debug)]
pub struct Route {
    method: Method,
    path: Path,
    action: Action,
}

impl Route {
    pub fn new<F>(method: Method, path: &'static str, action: F) -> Result<Self, SerwerError>
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        Ok(Self {
            method,
            path: Path::from_string(String::from(path)).unwrap(),
            action: Action::new(action),
        })
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn run_action(&self, request: Request) -> Response {
        self.action.run(request)
    }
}
