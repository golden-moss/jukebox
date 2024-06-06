use anyhow::Result;
use lofty::{file::TaggedFileExt, read_from_path, tag::Tag};
use rodio::{OutputStream, Sink};

struct SongMetadata {
    album: String,
    artist: String,
    comment: String,
    genre: String,
    disk: String,
    title: String,
    track: String,
    track_total: String,
}

pub fn new_sink() -> Sink {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    Box::leak(Box::new(stream));
    Sink::try_new(&stream_handle).unwrap()
}

pub fn read_tags(path: &str) -> Result<Vec<Tag>> {
    // This will guess the format from the extension
    let tagged_file = read_from_path(path)?;
    let mut tags = Vec::new();
    tags.extend_from_slice(tagged_file.tags());
    Ok(tags)
}

pub fn get_metadata_from_tags(tags: Vec<Tag>) -> SongMetadata {
    // for tag in tags {
    //     tag.pictures();
    // }
    unimplemented!()
}
