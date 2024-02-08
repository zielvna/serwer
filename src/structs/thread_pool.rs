use crate::{utils::macros::unwrap_error, Route, Worker};
use std::{
    net::TcpStream,
    sync::{mpsc, Arc, Mutex, RwLock},
};

#[derive(Debug)]
pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<TcpStream>,
}

impl ThreadPool {
    pub fn new(size: usize, routes: &Arc<RwLock<Vec<Route>>>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&routes)));
        }

        Self {
            _workers: workers,
            sender,
        }
    }

    pub fn handle_stream(&self, stream: TcpStream) {
        unwrap_error!(self.sender.send(stream), "Failed to send stream to worker");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{stream_from_bytes, Method};
    use std::{sync::RwLock, thread, time::Duration};

    #[test]
    fn test_new() {
        let routes = Arc::new(RwLock::new(vec![]));
        let pool = ThreadPool::new(4, &routes);
        assert_eq!(pool._workers.len(), 4);
    }

    #[test]
    fn test_handle_stream() {
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);

        let route = Route::new(Method::GET, "/", move |_, res| {
            let mut count = count_clone.lock().unwrap();
            *count += 1;
            res
        })
        .unwrap();
        let routes = Arc::new(RwLock::new(vec![route]));
        let pool = ThreadPool::new(4, &routes);

        let stream = stream_from_bytes(b"GET / HTTP/1.1\r\n\r\n");
        pool.handle_stream(stream);
        let stream = stream_from_bytes(b"GET / HTTP/1.1\r\n\r\n");
        pool.handle_stream(stream);
        let stream = stream_from_bytes(b"GET / HTTP/1.1\r\n\r\n");
        pool.handle_stream(stream);

        thread::sleep(Duration::from_millis(100));

        assert_eq!(*count.lock().unwrap(), 3);
    }
}
