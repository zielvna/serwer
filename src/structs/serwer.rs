use super::{Request, Response, Route};
use crate::enums::Method;
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Serwer {
    routes: Vec<Route>,
    port: Option<u16>,
    listener: Option<TcpListener>,
    public_path: Option<String>,
}

impl Serwer {
    pub fn new() -> Self {
        Self {
            routes: vec![],
            port: None,
            listener: None,
            public_path: None,
        }
    }

    pub fn get<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::GET, path, action).expect("Error while setting route"));
    }

    pub fn head<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::HEAD, path, action).expect("Error while setting route"));
    }

    pub fn post<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::POST, path, action).expect("Error while setting route"));
    }

    pub fn put<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::PUT, path, action).expect("Error while setting route"));
    }

    pub fn delete<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::DELETE, path, action).expect("Error while setting route"));
    }

    pub fn connect<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::CONNECT, path, action).expect("Error while setting route"));
    }

    pub fn options<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::OPTIONS, path, action).expect("Error while setting route"));
    }

    pub fn trace<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::TRACE, path, action).expect("Error while setting route"));
    }

    pub fn patch<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::PATCH, path, action).expect("Error while setting route"));
    }

    pub fn listen(&mut self, port: u16) {
        self.port = Some(port);
        self.listener = Some(
            TcpListener::bind(format!("127.0.0.1:{port}")).expect("Error while binding to a port"),
        );

        for stream in self
            .listener
            .as_ref()
            .expect("Error while listening to a port")
            .incoming()
        {
            let stream = stream.expect("Error while reading stream");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let request = Request::from_stream(&stream).expect("Error while reading request");

        let found = self.find_and_handle_route(&request, &mut stream);

        if !found {
            self.find_and_handle_file(&request, &mut stream);
        }
    }

    fn find_and_handle_route(&self, request: &Request, stream: &mut TcpStream) -> bool {
        for route in self.routes.iter() {
            if route.get_method() == &request.get_method() {
                let (matches, params) = route.get_path().matches(&request.get_path());

                if matches {
                    let mut request = request.clone();
                    request.set_params(params.expect("Error while setting params to a request"));

                    let response = route.run_action(request);
                    stream
                        .write_all(response.write().as_slice())
                        .expect("Error while writing response");

                    return true;
                }
            }
        }

        false
    }

    fn find_and_handle_file(&self, request: &Request, stream: &mut TcpStream) -> bool {
        let parsed_path = request.get_path().get_string().replacen("/", "", 1);
        let file_path = env::current_dir()
            .expect("Error while getting current directory")
            .join(
                self.public_path
                    .as_ref()
                    .expect("Error while getting public path"),
            )
            .join(parsed_path);

        if let Ok(mut file) = File::open(file_path) {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .expect("Error while reading file");

            let mut response = Response::new(&request.get_version());
            response.set_body_from_bytes(buffer);
            stream
                .write_all(response.write().as_slice())
                .expect("Error while writing response");

            true
        } else {
            false
        }
    }

    pub fn public(&mut self, path: &str) {
        let metadata = fs::metadata(&path).expect("Error while reading metadata");

        if !metadata.is_dir() {
            panic!("Public path is not a directory.");
        }

        self.public_path = Some(String::from(path));
    }
}
