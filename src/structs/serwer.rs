use super::{thread_pool::ThreadPool, Request, Response, Route};
use crate::enums::Method;
use crate::unwrap_error;
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

    #[track_caller]
    pub fn get<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::GET, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn head<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::HEAD, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn post<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::POST, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn put<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::PUT, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn delete<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::DELETE, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn connect<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::CONNECT, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn options<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::OPTIONS, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn trace<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::TRACE, path, action),
            "Error while setting route"
        ));
    }

    #[track_caller]
    pub fn patch<F>(&mut self, path: &'static str, action: F)
    where
        F: Fn(Request, Response) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::clone(&self.routes);
        let mut routes = routes.write().expect("Error while locking routes");

        routes.push(unwrap_error!(
            Route::new(Method::PATCH, path, action),
            "Error while setting route"
        ));
    }

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
