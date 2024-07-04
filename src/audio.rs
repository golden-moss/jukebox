use rodio::{OutputStream, Sink};

pub fn new_sink() -> Sink {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    Box::leak(Box::new(stream));
    Sink::try_new(&stream_handle).unwrap()
}
