use anyhow::Result;
use rodio::{source, Decoder, OutputStream, Sink};
use std::{
    io::{BufReader, Read, Seek},
    path::PathBuf,
};

use crate::PlaybackSettings;

pub fn new_sink(settings: PlaybackSettings) -> Sink {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    Box::leak(Box::new(stream));
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(settings.volume);
    sink.set_speed(settings.speed);

    sink
}

// pub fn new_source(path: PathBuf) -> Result<Decoder<Read + Seek + 'static>> {
//     let source = rodio::Decoder::new(BufReader::new(std::fs::File::open(path)?))?;
//     Ok(source)
// }
