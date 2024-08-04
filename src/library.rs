use anyhow::anyhow;
use anyhow::Result;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    // pub artist: String, // TODO refer to actual artists (and deal with multiple)
    pub artist: Artist,
    pub duration: Duration,
    pub album_title: Option<String>,
    pub file_path: PathBuf,
    pub year: u16,
    pub genre: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: Uuid,
    pub title: String,
    // pub artist: Vec<Uuid>, //TODO implement support for multiple artists
    pub artist: Artist,
    pub songs: Vec<Song>, // Vec (or HashMap?) of `Song.id`s - NO EMPTY ALBUMS (hope this is not an edge case lmao)
    pub year: u16,
    pub genre: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
}
// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
// pub struct Artist {
//     pub id: Uuid,
//     pub name: String,
//     pub albums: Option<Vec<Uuid>>, // Vec (or HashMap?) of `Album.id`s
//     pub songs: Option<Vec<Uuid>>,  // Vec (or HashMap?) of `Song.id`s
//     pub genre: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub songs: HashMap<Uuid, Song>,
    pub albums: HashMap<Uuid, Album>,
    // pub album_songs: HashMap<Uuid, Vec<Uuid>>, // album_id -> [song_ids] //TODO get rid of this, use Album instead
}

impl Song {
    pub fn new_from_file(file_path: PathBuf) -> Result<Self> {
        let tagged_file = Probe::open(&file_path)?.read()?;

        match tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
        {
            Some(tag) => {
                let id = Uuid::new_v4();
                let unknown_tag = std::borrow::Cow::Borrowed("Unknown");
                let title = tag.title().unwrap_or(unknown_tag.clone()).to_string();
                let album_title = Album::only_title(tag.album());
                let artist = tag.artist().unwrap_or(unknown_tag.clone()).to_string();
                let year = tag.year().unwrap_or(0) as u16;
                let genre = tag.genre().unwrap_or(unknown_tag.clone()).to_string();
                let duration = tagged_file.properties().duration();

                // TODO check if Album exists (by name and artist)
                // TODO create if does not, append if does

                // let album_id = Album::get_or_create_from_song(id).id;
                // let album_id = None; // TODO for now, set all to None and apply id later, not quite sure how to deal with creating Albums right now

                Ok(Song {
                    id,
                    title,
                    album_title,
                    artist: Artist::new(artist),
                    duration,
                    file_path,
                    year,
                    genre,
                })
            }
            None => {
                // TODO handle valid songs that have no tags
                // Err(anyhow!("not an audio file"))
                Ok(Song::default())
            }
        }
    }
}

impl Album {
    pub fn new(title: String, artist: Artist, year: u16, genre: String) -> Self {
        Album {
            id: Uuid::new_v4(),
            title,
            artist,
            songs: Vec::new(),
            year,
            genre,
        }
    }

    pub fn get_or_create_from_song(song: &Song) -> Self {
        Album {
            id: Uuid::new_v4(),
            title: song.album_title.unwrap(),
            artist: song.artist,
            songs: Vec::new(),
            year: (),
            genre: (),
        }
    }

    pub fn get_album_songs(&self) -> Vec<Song> {
        self.songs.clone()
    }

    pub fn only_title(maybe_album: Option<Cow<str>>) -> Option<String> {
        match maybe_album {
            Some(title) => Some(title.to_string()),
            None => None,
        }
    }
}

impl Artist {
    pub fn new(name: String) -> Self {
        Artist {
            id: Uuid::new_v4(),
            name,
        }
    }
}

impl Library {
    pub fn new() -> Self {
        Library {
            songs: HashMap::new(),
            albums: HashMap::new(),
            // album_songs: HashMap::new(),
        }
    }

    fn add_song(&mut self, song: Song) -> Result<()> {
        self.songs.insert(song.id.clone(), song);

        Ok(())
    }

    fn add_album(&mut self, album: Album) -> Result<()> {
        self.albums.insert(album.id.clone(), album);

        Ok(())
    }

    pub fn import_dir(&mut self, folder_path: &str) -> Result<()> {
        // TODO check for existing dupes based on filepath, duration, other tags, and ideally AcoustID but I have *no* clue how to implement that.
        for entry in WalkDir::new(folder_path) {
            match entry {
                Ok(file) => {
                    // println!("entry: {:?}", file);
                    // println!("{:?}", entry.file_name());
                    if !file.file_type().is_file() {
                        match file.clone().into_path().extension() {
                            Some(extension) => {
                                if extension == "flac"
                                    || extension == "ogg"
                                    || extension == "mp3"
                                    || extension == "wav"
                                    || extension == "acc"
                                {
                                    println!("ADDING SONG: {:?}", file.clone().file_name());
                                    self.add_song(Song::new_from_file(file.into_path())?)?;
                                }
                            }
                            None => (),
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
        }

        // TODO sort into Albums?

        Ok(())
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let toml = toml::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(toml.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(file_path: &str) -> Result<Library> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let library: Library = toml::from_str(&contents)?;
        Ok(library)
    }
}
