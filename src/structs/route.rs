use super::{Action, Request, Response};
use crate::{
    enums::{Method, SerwerError},
    utils::validate_path,
};

#[derive(Debug)]
pub struct Route {
    method: Method,
    path: String,
    action: Action,
}

impl Route {
    pub fn new<F>(method: Method, path: &'static str, action: F) -> Result<Self, SerwerError>
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        validate_path(&path)?;

        Ok(Self {
            method,
            path: String::from(path),
            action: Action::new(action),
        })
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn run_action(&self, request: Request) -> Response {
        self.action.run(request)
    }
}
