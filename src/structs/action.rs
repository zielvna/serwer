use super::{Request, Response};

pub struct Action {
    func: Box<dyn Fn(Request) -> Response>,
}

impl Action {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(Request) -> Response + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }

    pub fn run(&self, request: Request) -> Response {
        (self.func)(request)
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
