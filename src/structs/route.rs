use crate::{Action, Method, Path, Request, Response, SerwerError};

#[derive(Debug)]
pub struct Route {
    method: Method,
    path: Path,
    action: Action,
}

impl Route {
    pub fn new<F>(method: Method, path: &'static str, action: F) -> Result<Self, SerwerError>
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        Ok(Self {
            method,
            path: Path::from_string(&String::from(path))?,
            action: Action::new(action),
        })
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn run_action(&self, request: Request) -> Response {
        self.action.run(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request_from_bytes;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_new_closure_run() {
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);

        let route = Route::new(Method::GET, "/", move |_, res| {
            let mut count = count_clone.lock().unwrap();
            *count += 1;
            res
        })
        .unwrap();

        let request = request_from_bytes(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        route.run_action(request.clone());
        route.run_action(request.clone());
        route.run_action(request);

        assert_eq!(*count.lock().unwrap(), 3);
    }
}
