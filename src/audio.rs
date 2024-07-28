use rodio::{OutputStream, Sink};

mod media_controls;

use crate::PlaybackSettings;

pub fn new_sink(settings: PlaybackSettings) -> Sink {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    Box::leak(Box::new(stream));
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(settings.volume);
    sink.set_speed(settings.speed);

    sink
}
