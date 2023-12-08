use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

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
    }
}

#[derive(Debug)]
pub struct Route {
    method: HTTPMethod,
    path: String,
}

impl Route {
    pub fn new(method: HTTPMethod, path: String) -> Self {
        Self { method, path }
    }
}

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
}
