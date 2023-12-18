use super::{Request, Response, Route};
use crate::enums::Method;
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
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

    pub fn add_route<F>(&mut self, method: Method, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes.push(Route::new(method, path, action).unwrap());
    }

    pub fn get<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::GET, path, action).unwrap());
    }

    pub fn post<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + 'static,
    {
        self.routes
            .push(Route::new(Method::POST, path, action).unwrap());
    }

    pub fn listen(&mut self, port: u16) {
        self.port = Some(port);
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        self.listener = Some(TcpListener::bind(address).unwrap());

        for stream in self.listener.as_ref().unwrap().incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut request = Request::from_stream(&stream).unwrap();

        for route in self.routes.iter() {
            let (matches, params) = route.get_path().matches(&request.get_path());

            if route.get_method() == &request.get_method() && matches {
                request.set_params(params.unwrap());

                let response = route.run_action(request);
                stream.write_all(response.write().as_slice()).unwrap();

                return;
            }
        }

        let file_path = env::current_dir()
            .unwrap()
            .join(self.public_path.as_ref().unwrap())
            .join(request.get_path().get_string());

        if let Ok(mut file) = File::open(file_path) {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();

            let mut response = Response::default();
            response.set_body(buffer);
            stream.write_all(response.write().as_slice()).unwrap();
        };
    }

    pub fn public(&mut self, path: &str) {
        let metadata = fs::metadata(&path).unwrap_or_else(|error| {
            panic!(
                "Error while trying to read metadata from a path: {:?}",
                error.to_string()
            )
        });

        if !metadata.is_dir() {
            panic!("Path is not a directory.");
        }

        self.public_path = Some(String::from(path));
    }
}
