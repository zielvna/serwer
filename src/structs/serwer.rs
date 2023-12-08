use std::{
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

use crate::enums::HTTPMethod;

use super::Route;

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
            if route.get_method() == &method && route.get_path() == path {
                let action_result = route.run_action();
                let length = action_result.len();
                let response =
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{action_result}");
                stream.write_all(response.as_bytes()).unwrap();
                break;
            }
        }
    }
}
