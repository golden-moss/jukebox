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
    pub title: String,
    // pub artist: Vec<Uuid>, //TODO implement support for multiple artists
    pub artist: Artist,
    pub songs: Vec<Song>, // Vec (or HashMap?) of `Song.id`s - NO EMPTY ALBUMS (hope this is not an edge case lmao)
    pub year: u16,
    pub genre: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub albums: Option<Vec<Uuid>>,
    pub songs: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub songs: HashMap<Uuid, Song>,
    pub albums: HashMap<Uuid, Album>,
    pub artists: HashMap<Uuid, Artist>,
}

impl Song {
    pub fn new_from_file(file_path: PathBuf) -> Result<Self> {
        let tagged_file = Probe::open(&file_path)?.read()?;

        match tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
        {
            Some(tag) => {
                let unknown_tag = std::borrow::Cow::Borrowed("Unknown");
                let title = tag.title().unwrap_or(unknown_tag.clone()).to_string();
                let album_title = Album::only_title(tag.album());
                let artist = tag.artist().unwrap_or(unknown_tag.clone()).to_string();
                let year = tag.year().unwrap_or(0) as u16;
                let genre = tag.genre().unwrap_or(unknown_tag.clone()).to_string();
                let duration = tagged_file.properties().duration();

                Ok(Song {
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
                Ok(Song::default())
            }
        }
    }
}

impl Album {
    pub fn new(title: String, artist: Artist, year: u16, genre: String) -> Self {
        Album {
            title,
            artist,
            songs: Vec::new(),
            year,
            genre,
        }
    }

    pub fn create_from_song(song: &Song) -> Self {
        Album {
            title: song.clone().album_title.unwrap().to_owned(),
            artist: song.artist.clone(),
            songs: Vec::new(),
            year: song.year,
            genre: song.genre.to_owned(),
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
            name,
            albums: None,
            songs: None,
        }
    }
}

impl Library {
    pub fn new() -> Self {
        Library {
            songs: HashMap::new(),
            albums: HashMap::new(),
            artists: HashMap::new(),
        }
    }

    fn add_song(&mut self, song: Song) -> Result<()> {
        // TODO check for duplicates (by name, possibly album, and artist)
        self.songs.insert(Uuid::new_v4(), song);

        Ok(())
    }

    fn add_album(&mut self, album: Album) -> Result<()> {
        // TODO check if Album exists (by name and artist)
        // TODO create if does not, append if does

        // let album_id = Album::get_or_create_from_song(id).id;
        // let album_id = None; // TODO for now, set all to None and apply id later, not quite sure how to deal with creating Albums right now

        // self.albums.insert(album.id.clone(), album);
        self.albums.insert(Uuid::new_v4(), album);

        Ok(())
    }

    fn add_artist(&mut self, artist: Artist) -> Result<()> {
        // TODO check for duplicates
        self.artists.insert(Uuid::new_v4(), artist);

        Ok(())
    }

    pub fn import_dir(&mut self, folder_path: &str) -> Result<()> {
        for entry in WalkDir::new(folder_path) {
            // TODO check for existing dupes based on filepath, duration, other tags, and ideally AcoustID but I have *no* clue how to implement that.
            match entry {
                Ok(file) => {
                    // println!("entry: {:?}", file);
                    if file.file_type().is_file() {
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
        // for (_id, song) in self.songs.clone() {
        //     self.add_album(Album::create_from_song(&song))?;
        // }

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
