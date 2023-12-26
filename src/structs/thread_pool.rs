use crate::structs::{Route, Worker};
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
    pub fn new(
        size: usize,
        routes: Arc<RwLock<Vec<Route>>>,
        public_path: Arc<RwLock<Option<String>>>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&routes),
                Arc::clone(&public_path),
            ));
        }

        Self {
            _workers: workers,
            sender,
        }
    }

    pub fn handle_stream(&self, stream: TcpStream) {
        self.sender.send(stream).unwrap();
    }
}
