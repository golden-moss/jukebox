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
    pub artists: Vec<Artist>,
    pub duration: Duration,
    pub album: Option<Album>,
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
    // pub songs: Vec<Song>, // Vec (or HashMap?) of `Song.id`s - NO EMPTY ALBUMS (hope this is not an edge case lmao)
    // pub year: u16,
    // pub genre: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    // pub albums: Option<Vec<Uuid>>,
    // pub songs: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub songs: HashMap<Uuid, Song>,
    pub albums: HashMap<Uuid, Album>,
    pub artists: HashMap<Uuid, Artist>,
}

impl Song {
    pub fn new(file_path: PathBuf) -> Self {
        match Probe::open(&file_path) {
            Ok(tagged_file) => {
                match tagged_file.read() {
                    Ok(tagged_file) => {
                        match tagged_file
                            .primary_tag()
                            .or_else(|| tagged_file.first_tag())
                        {
                            Some(tag) => {
                                let unknown_tag = std::borrow::Cow::Borrowed("Unknown");
                                let tag_title =
                                    tag.title().unwrap_or(unknown_tag.clone()).to_string();
                                let tag_artist =
                                    tag.artist().unwrap_or(unknown_tag.clone()).to_string();
                                let tag_year = tag.year().unwrap_or(0) as u16;
                                let tag_genre =
                                    tag.genre().unwrap_or(unknown_tag.clone()).to_string();
                                let tag_duration = tagged_file.properties().duration();

                                let artist = Artist::new(tag_artist.clone());

                                let album = Album::new(
                                    Album::try_to_get_title(tag.album()),
                                    artist.clone(),
                                );

                                Song {
                                    title: tag_title,
                                    album,
                                    artists: vec![artist],
                                    duration: tag_duration,
                                    file_path,
                                    year: tag_year,
                                    genre: tag_genre,
                                }
                            }
                            None => {
                                // TODO handle valid songs that have no tags
                                // TODO this is a shitty hack, properly deal with errors
                                Song::default()
                            }
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        // TODO this is a shitty hack, properly deal with errors
                        Song::default()
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
                // TODO this is a shitty hack, properly deal with errors
                Song::default()
            }
        }
    }
}

impl Album {
    pub fn new(title: Option<String>, artist: Artist) -> Option<Self> {
        match title {
            Some(title) => Some(Album {
                id: Uuid::new_v4(),
                title,
                artist,
            }),
            None => None,
        }
    }

    pub fn try_to_get_title(maybe_album: Option<Cow<str>>) -> Option<String> {
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
            artists: HashMap::new(),
        }
    }

    // fn load(&self, path: ) -> Result<(), String> {
    //     let load_path = self.global_settings.library_file.clone();
    //     let library = Arc::clone(&self.music_library);
    //     Library::read_from_file(&load_path)
    //         .map(|new_lib| {
    //             let mut lib = library.lock();
    //             *lib = new_lib;
    //         })
    //         .map_err(|e| e.to_string())
    // }

    fn add_song(&mut self, song: Song) -> Result<()> {
        // TODO check for duplicates (by name, possibly album, and artist)
        self.songs.insert(Uuid::new_v4(), song);

        Ok(())
    }

    // fn add_album(&mut self, album: Album) -> Result<()> {
    //     // TODO check if Album exists (by name and artist)
    //     // TODO create if does not, append if does

    //     // let album_id = Album::get_or_create_from_song(id).id;
    //     // let album_id = None; // TODO for now, set all to None and apply id later, not quite sure how to deal with creating Albums right now

    //     // self.albums.insert(album.id.clone(), album);
    //     self.albums.insert(Uuid::new_v4(), album);

    //     Ok(())
    // }

    // fn add_artist(&mut self, artist: Artist) -> Result<()> {
    //     // TODO check for duplicates
    //     self.artists.insert(Uuid::new_v4(), artist);

    //     Ok(())
    // }

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
                                    self.add_song(Song::new(file.into_path()))?;
                                }
                            }
                            None => (),
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
        }

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
