use std::{
    io::{BufRead, BufReader, Write},
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

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
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
        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let parts: Vec<&str> = http_request[0].split(' ').collect();

        let method = match parts[0] {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            _ => panic!("HTTP method of a request not found."),
        };
        let path = parts[1];

        for route in self.routes.iter() {
            if route.method == method && route.path == path {
                let action_result = route.action.run();
                let length = action_result.len();
                let response =
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{action_result}");
                stream.write_all(response.as_bytes()).unwrap();
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Route {
    method: HTTPMethod,
    path: String,
    action: Action,
}

impl Route {
    pub fn new(method: HTTPMethod, path: String, action: Action) -> Self {
        Self {
            method,
            path,
            action,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
}

pub struct Action {
    func: Box<dyn Fn() -> String>,
}

impl Action {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }

    pub fn run(&self) -> String {
        (self.func)()
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
