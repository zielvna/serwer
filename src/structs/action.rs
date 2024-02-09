use crate::{Request, Response, Version};

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
        let response = Response::new(&Version::HTTP_1_1);
        (self.func)(request, response)
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request_from_bytes;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_closure_run() {
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);

        let action = Action::new(move |_, response| {
            let mut count = count_clone.lock().unwrap();
            *count += 1;
            response
        });

        let request = request_from_bytes(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        action.run(request.clone());
        action.run(request.clone());
        action.run(request);

        assert_eq!(*count.lock().unwrap(), 3);
    }

    #[test]
    fn test_display() {
        let action = Action::new(|_, response| response);
        assert_eq!(format!("{:?}", action), "function");
    }
}
