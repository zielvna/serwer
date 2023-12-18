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
            path: Path::from_string(&String::from(path))?,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::Cell,
        io::Write,
        net::{TcpListener, TcpStream},
        rc::Rc,
        thread,
    };

    fn request_from_bytes(data: &[u8]) -> Result<Request, SerwerError> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();

        let buf = data.to_owned();

        thread::spawn(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            stream.write_all(&buf).unwrap();
        });

        let (stream, _) = listener.accept().unwrap();
        Request::from_stream(&stream)
    }

    #[test]
    fn test_new_closure_run() {
        let count = Rc::new(Cell::new(0));
        let count_clone = Rc::clone(&count);

        let route = Route::new(Method::GET, "/", move |_, res| {
            count_clone.set(count_clone.get() + 1);
            res
        })
        .unwrap();

        let request = request_from_bytes(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        route.run_action(request.clone());
        route.run_action(request.clone());
        route.run_action(request);

        assert_eq!(count.get(), 3);
    }
}
