use super::{Request, Response, Route};
use crate::enums::Method;
use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Serwer {
    routes: Vec<Route>,
    port: Option<u16>,
    listener: Option<TcpListener>,
}

impl Serwer {
    pub fn new() -> Self {
        Self {
            routes: vec![],
            port: None,
            listener: None,
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
        let request = Request::from_stream(&mut stream).unwrap();

        for route in self.routes.iter() {
            if route.get_method() == &request.get_method() && route.get_path() == request.get_path()
            {
                let response = route.run_action(request);
                let body = response.get_body();
                let length = body.len();
                let status_code = response.get_status_code().to_string();
                let response =
                    format!("HTTP/1.1 {status_code}\r\nContent-Length: {length}\r\n\r\n{body}");
                stream.write_all(response.as_bytes()).unwrap();
                break;
            }
        }
    }
}
