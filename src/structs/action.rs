use crate::enums::StatusCode;

use super::{Request, Response};

pub struct Action {
    func: Box<dyn Fn(Request, Response) -> Response>,
}

impl Action {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }

    pub fn run(&self, request: Request) -> Response {
        let response = Response::new(StatusCode::OK, String::from(""));
        (self.func)(request, response)
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
