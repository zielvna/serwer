use crate::{print_error, unwrap_error, Method, Request, Response, Route, StatusCode, Version};
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
