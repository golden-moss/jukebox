use anyhow::Result;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub duration: Duration, // in seconds
    pub album_id: Option<u64>,
    pub file_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub year: u16,
    pub genre: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub songs: HashMap<u64, Song>,
    pub albums: HashMap<u64, Album>,
    pub album_songs: HashMap<u64, Vec<u64>>, // album_id -> [song_ids]
    pub next_song_id: u64,                   // used when creating/importing new songs
    pub next_album_id: u64,                  // used when creating/importing new albums
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: 0,
            title: "default title".into(),
            artist: "Default Artist".into(),
            duration: Duration::ZERO,
            album_id: None,
            file_path: PathBuf::new(),
        }
    }
}

// impl Song {
//   pub fn new(id) -> Self {
//     Song { id: id, title: (), artist: (), duration: (), album_id: () }
//   }
// }

impl Library {
    pub fn new() -> Self {
        Library {
            songs: HashMap::new(),
            albums: HashMap::new(),
            album_songs: HashMap::new(),
            next_song_id: 1,
            next_album_id: 1,
        }
    }

    fn add_song(&mut self, song: Song) {
        if let Some(album_id) = song.album_id {
            self.album_songs.entry(album_id).or_default().push(song.id);
        }
        self.songs.insert(song.id, song);
    }

    fn add_album(&mut self, album: Album) -> u64 {
        let id = self.next_album_id;
        self.next_album_id += 1;
        self.albums.insert(id, album);
        id
    }

    pub fn get_song(&self, id: u64) -> Option<&Song> {
        self.songs.get(&id)
    }

    pub fn get_album(&self, id: u64) -> Option<&Album> {
        self.albums.get(&id)
    }

    pub fn get_or_create_album(
        &mut self,
        title: String,
        artist: String,
        year: u16,
        genre: String,
    ) -> u64 {
        for (id, album) in &self.albums {
            if album.title == title && album.artist == artist {
                return *id;
            }
        }

        let new_album = Album {
            id: self.next_album_id,
            title,
            artist,
            year,
            genre,
        };
        self.add_album(new_album)
    }

    pub fn create_songs_from_folder(
        &mut self,
        folder_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(song) = self.create_song_from_file(entry.path()) {
                    self.add_song(song);
                }
            }
        }
        Ok(())
    }

    pub fn create_song_from_file(&mut self, file_path: &Path) -> Option<Song> {
        let tagged_file = Probe::open(file_path).ok()?.read().ok()?;

        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())?;

        let unknown_tag = std::borrow::Cow::Borrowed("Unknown");
        let title = tag.title().unwrap_or(unknown_tag.clone()).to_string();
        let artist = tag.artist().unwrap_or(unknown_tag.clone()).to_string();
        let album_title = tag.album().unwrap_or(unknown_tag.clone()).to_string();
        let year = tag.year().unwrap_or(0) as u16;
        let genre = tag.genre().unwrap_or(unknown_tag.clone()).to_string();

        let duration = tagged_file.properties().duration();

        let album_id = self.get_or_create_album(album_title, artist.clone(), year, genre);

        let song = Song {
            id: self.next_song_id,
            title,
            artist,
            duration,
            album_id: Some(album_id),
            file_path: file_path.to_owned(),
        };

        self.next_song_id += 1;

        println!("{:#?}", song);

        Some(song)
    }

    pub fn get_album_songs(&self, album_id: u64) -> Vec<&Song> {
        self.album_songs
            .get(&album_id)
            .map(|song_ids| {
                song_ids
                    .iter()
                    .filter_map(|id| self.songs.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let json = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(file_path: &str) -> Result<Library> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let library: Library = serde_json::from_str(&contents)?;
        Ok(library)
    }
}
