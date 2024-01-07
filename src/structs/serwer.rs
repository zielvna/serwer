use crate::{
    unwrap_error,
    utils::{generate_route, unwrap_none},
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
        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            default_panic(info);
            std::process::exit(1);
        }));

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
            unwrap_error!(
                thread::available_parallelism(),
                "Error while trying to get number of available threads"
            )
            .get(),
            &self.routes,
        ));

        for stream in unwrap_none!(
            self.listener.as_ref(),
            "Error while listening to incoming stream"
        )
        .incoming()
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
