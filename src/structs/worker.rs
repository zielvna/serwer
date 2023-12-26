use crate::{structs::route::Route, Request, Response, StatusCode, Version};
use std::{
    env,
    fs::File,
    io::{Read, Write},
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
        public_path: Arc<RwLock<Option<String>>>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let mut stream = receiver.lock().unwrap().recv().unwrap();

            let response = match Self::handle_stream(&stream, &routes, &public_path) {
                Some(response) => response,
                None => {
                    let mut response = Response::new(&Version::HTTP_1_0);
                    response.set_status_code(StatusCode::NotFound);
                    response
                }
            };

            stream
                .write_all(response.write().as_slice())
                .expect("Error while writing response");
        });

        Self {
            _id: id,
            _thread: thread,
        }
    }

    fn handle_stream(
        stream: &TcpStream,
        routes: &Arc<RwLock<Vec<Route>>>,
        public_path: &Arc<RwLock<Option<String>>>,
    ) -> Option<Response> {
        let request = Request::from_stream(&stream).expect("Error while reading request");

        let response = Self::find_and_handle_route(&request, routes);

        if response.is_some() {
            return response;
        }

        let response = Self::find_and_handle_file(&request, public_path);

        if response.is_some() {
            return response;
        }

        None
    }

    fn find_and_handle_route(
        request: &Request,
        routes: &Arc<RwLock<Vec<Route>>>,
    ) -> Option<Response> {
        for route in routes.read().expect("Error while reading routes").iter() {
            if route.get_method() == &request.get_method() {
                let (matches, params) = route.get_path().matches(&request.get_path());

                if matches {
                    let mut request = request.clone();
                    request.set_params(params.expect("Error while setting params to a request"));

                    route.run_action(request);
                }
            }
        }

        None
    }

    fn find_and_handle_file(
        request: &Request,
        public_path: &Arc<RwLock<Option<String>>>,
    ) -> Option<Response> {
        let path = public_path.read().expect("Error while reading public path");

        if let Some(path) = path.as_ref() {
            let parsed_path = request.get_path().get_string().replacen("/", "", 1);
            let file_path = env::current_dir()
                .expect("Error while getting current directory")
                .join(path)
                .join(parsed_path);

            if let Ok(mut file) = File::open(file_path) {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .expect("Error while reading file");

                let mut response = Response::new(&request.get_version());
                response.set_body_from_bytes(buffer);

                Some(response)
            } else {
                None
            }
        } else {
            None
        }
    }
}
