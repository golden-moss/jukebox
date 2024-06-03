use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::Path};

pub fn new_sink() -> Sink {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    Box::leak(Box::new(stream));
    Sink::try_new(&stream_handle).unwrap()
}
