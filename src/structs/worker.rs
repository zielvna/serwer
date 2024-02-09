use crate::{
    utils::macros::{print_error, unwrap_error},
    Method, Request, Response, Route, StatusCode, Version,
};
use std::{
    io::Write,
    net::TcpStream,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

#[derive(Debug)]
pub struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<TcpStream>>>,
        routes: Arc<RwLock<Vec<Route>>>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let mut stream = unwrap_error!(
                unwrap_error!(receiver.lock(), "Failed to lock receiver").recv(),
                "Failed to receive stream from receiver"
            );

            let response = Self::handle_stream(&stream, &routes);

            print_error!(
                stream.write_all(response.write().as_slice()),
                "Error while writing response"
            );
        });

        Self {
            _id: id,
            _thread: thread,
        }
    }

    fn handle_stream(stream: &TcpStream, routes: &Arc<RwLock<Vec<Route>>>) -> Response {
        let request = Request::from_stream(&stream);

        if let Ok(request) = request {
            for route in unwrap_error!(routes.read(), "Error while reading routes").iter() {
                if route.method() == &request.method() || route.method() == &Method::ALL {
                    let (matches, params) = route.path().matches(&request.path());

                    if matches {
                        let mut request = request.clone();
                        request.set_params(params.unwrap());

                        return route.run_action(request);
                    }
                }
            }

            let mut response = Response::new(&Version::HTTP_1_1);
            response.set_status_code(StatusCode::NotFound);

            response
        } else {
            print_error!(request, "Error while reading request");

            let mut response = Response::new(&Version::HTTP_1_1);
            response.set_status_code(StatusCode::BadRequest);

            response
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_from_bytes;

    #[test]
    fn test_new() {
        let routes = Arc::new(RwLock::new(vec![]));
        let (_, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let worker = Worker::new(0, receiver, routes);
        assert_eq!(worker._id, 0);
    }

    #[test]
    fn test_handle_stream() {
        let route = Route::new(Method::GET, "/", move |_, mut res| {
            res.set(StatusCode::OK, "Hello World".to_string());
            res
        })
        .unwrap();
        let routes = Arc::new(RwLock::new(vec![route]));

        let stream = stream_from_bytes(b"GET / HTTP/1.1\r\n\r\n");
        let response = Worker::handle_stream(&stream, &routes);

        assert_eq!(
            String::from_utf8_lossy(response.write().as_slice()),
            "HTTP/1.1 200 OK\r\ncontent-length: 11\r\n\r\nHello World"
        );
    }

    #[test]
    fn test_handle_stream_bad_request() {
        let route = Route::new(Method::GET, "/", move |_, mut res| {
            res.set(StatusCode::OK, "Hello World".to_string());
            res
        })
        .unwrap();
        let routes = Arc::new(RwLock::new(vec![route]));

        let stream = stream_from_bytes(b"GET /\r\n\r\n");
        let response = Worker::handle_stream(&stream, &routes);

        assert_eq!(
            String::from_utf8_lossy(response.write().as_slice()),
            "HTTP/1.1 400 Bad Request\r\n\r\n"
        );
    }

    #[test]
    fn test_handle_stream_not_found() {
        let route = Route::new(Method::GET, "/hello", move |_, mut res| {
            res.set(StatusCode::OK, "Hello World".to_string());
            res
        })
        .unwrap();
        let routes = Arc::new(RwLock::new(vec![route]));

        let stream = stream_from_bytes(b"GET / HTTP/1.1\r\n\r\n");
        let response = Worker::handle_stream(&stream, &routes);

        assert_eq!(
            String::from_utf8_lossy(response.write().as_slice()),
            "HTTP/1.1 404 Not Found\r\n\r\n"
        );
    }
}
