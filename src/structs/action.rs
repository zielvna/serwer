use crate::{Request, Response};

pub struct Action {
    func: Box<dyn Fn(Request, Response) -> Response + Send + Sync + 'static>,
}

impl Action {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }

    pub fn run(&self, request: Request) -> Response {
        let response = Response::new(&request.version());
        (self.func)(request, response)
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
