use crate::{
    utils::macros::{custom_panic, generate_route, unwrap_error, unwrap_none},
    Method, Request, Response, Route, ThreadPool,
};
use std::{
    net::TcpListener,
    sync::{Arc, RwLock},
    thread,
};

#[derive(Debug)]
pub struct Serwer {
    routes: Arc<RwLock<Vec<Route>>>,
    listener: Option<TcpListener>,
    thread_pool: Option<ThreadPool>,
}

impl Serwer {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(vec![])),
            listener: None,
            thread_pool: None,
        }
    }

    generate_route!(all, Method::ALL);
    generate_route!(get, Method::GET);
    generate_route!(head, Method::HEAD);
    generate_route!(post, Method::POST);
    generate_route!(put, Method::PUT);
    generate_route!(delete, Method::DELETE);
    generate_route!(connect, Method::CONNECT);
    generate_route!(options, Method::OPTIONS);
    generate_route!(trace, Method::TRACE);
    generate_route!(patch, Method::PATCH);

    pub fn route_exists(&self, method: &Method, path: &str) -> bool {
        for route in self.routes.read().unwrap().iter() {
            if route.method() == method && route.path().original_url() == path {
                return true;
            }
        }

        false
    }

    #[track_caller]
    pub fn listen(&mut self, port: u16) {
        self.listener = Some(unwrap_error!(
            TcpListener::bind(format!("127.0.0.1:{port}")),
            "Error while binding to a port"
        ));

        self.thread_pool = Some(ThreadPool::new(
            unwrap_error!(
                thread::available_parallelism(),
                "Error while trying to get number of available threads"
            )
            .get(),
            &self.routes,
        ));

        for stream in
            unwrap_none!(self.listener.as_ref(), "Error while trying to get listener").incoming()
        {
            let stream = unwrap_error!(stream, "Error while reading stream");
            unwrap_none!(
                self.thread_pool.as_ref(),
                "Error while trying to get thread pool"
            )
            .handle_stream(stream);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port;

    #[test]
    fn test_new() {
        let mut serwer = Serwer::new();

        assert_eq!(serwer.routes.read().unwrap().len(), 0);
        assert!(serwer.listener.is_none());
        assert!(serwer.thread_pool.is_none());

        serwer.get("/", |_, res| res);

        assert_eq!(serwer.routes.read().unwrap().len(), 1);
    }

    #[test]
    fn test_listen() {
        thread::spawn(|| {
            let mut serwer = Serwer::new();

            serwer.listen(port());
        });

        thread::sleep(std::time::Duration::from_millis(100));
    }

    #[test]
    #[should_panic]
    fn test_listen_port_already_bound() {
        let port = port();

        thread::spawn(move || {
            let mut serwer = Serwer::new();

            serwer.listen(port);
        });

        thread::sleep(std::time::Duration::from_millis(100));

        let thread = thread::spawn(move || {
            let mut serwer = Serwer::new();

            serwer.listen(port);
        });

        thread::sleep(std::time::Duration::from_millis(100));

        thread.join().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_add_two_identical_routes() {
        let port = port();

        let thread = thread::spawn(move || {
            let mut serwer = Serwer::new();

            serwer.get("/", |_, res| res);
            serwer.get("/", |_, res| res);

            serwer.listen(port);
        });

        thread::sleep(std::time::Duration::from_millis(100));

        thread.join().unwrap();
    }
}
