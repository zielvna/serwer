use crate::{Request, SerwerError};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::atomic::{AtomicU16, Ordering},
    thread,
};

pub fn stream_from_bytes(data: &[u8]) -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();

    let buf = data.to_owned();

    thread::spawn(move || {
        let mut stream = TcpStream::connect(address).unwrap();
        stream.write_all(&buf).unwrap();
    });

    let (stream, _) = listener.accept().unwrap();
    stream
}

pub fn request_from_bytes(data: &[u8]) -> Result<Request, SerwerError> {
    let stream = stream_from_bytes(data);

    Request::from_stream(&stream)
}

static COUNTER: AtomicU16 = AtomicU16::new(29170);

pub fn port() -> u16 {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
