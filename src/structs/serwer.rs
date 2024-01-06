use crate::{unwrap_error, utils::generate_route, Method, Request, Response, Route, ThreadPool};
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

    generate_route!(get, Method::GET);
    generate_route!(head, Method::HEAD);
    generate_route!(post, Method::POST);
    generate_route!(put, Method::PUT);
    generate_route!(delete, Method::DELETE);
    generate_route!(connect, Method::CONNECT);
    generate_route!(options, Method::OPTIONS);
    generate_route!(trace, Method::TRACE);
    generate_route!(patch, Method::PATCH);

    #[track_caller]
    pub fn listen(&mut self, port: u16) {
        self.listener = Some(unwrap_error!(
            TcpListener::bind(format!("127.0.0.1:{port}")),
            "Error while binding to a port"
        ));

        self.thread_pool = Some(ThreadPool::new(
            thread::available_parallelism()
                .expect("Error while trying to get available threads")
                .get(),
            Arc::clone(&self.routes),
        ));

        for stream in self
            .listener
            .as_ref()
            .expect("Error while listening to a port")
            .incoming()
        {
            let stream = stream.expect("Error while reading stream");
            self.thread_pool
                .as_ref()
                .expect("Error while trying to get thread pool")
                .handle_stream(stream);
        }
    }
}
