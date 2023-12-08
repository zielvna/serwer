pub struct Serwer {
    routes: Vec<Route>,
    port: Option<u16>,
}

impl Serwer {
    pub fn new() -> Self {
        Self {
            routes: vec![],
            port: None,
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn listen(&mut self, port: u16) {
        self.port = Some(port)
    }
}

pub struct Route {
    method: HTTPMethod,
    path: String,
}

impl Route {
    pub fn new(method: HTTPMethod, path: String) -> Self {
        Self { method, path }
    }
}

pub enum HTTPMethod {
    GET,
    POST,
}
