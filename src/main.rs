// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod library;

use anyhow::Result;
use library::{Library, Song};
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::{
    io::BufReader,
    sync::{Arc, Mutex},
};

use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Application, Command, Element, Length, Settings, Size, Subscription, Theme,
};

#[derive(Debug, Serialize, Deserialize)]
struct GlobalSettings {
    folder_to_scan: String, // TODO add ability to scan multiple folders
    library_file: String,   // where the serialized library is saved
                            // theme: VisualTheme
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            folder_to_scan: String::from("./"),
            library_file: String::from("library.toml"),
        }
    }
}

struct PlaybackSettings {
    volume: f32, // lets leave this at 1.0 for now
    speed: f32,  // lets leave this at 1.0 for now
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            speed: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePlayback,
    PreviousSong,
    NextSong,
    AddTestSongToQueue,
    Scan,
    ScanComplete(Result<(), String>),
    PickSong(u64),
    SaveLibrary,
    SaveComplete(Result<(), String>),
    LoadLibrary,
    LoadComplete(Result<(), String>),
}

struct Jukebox {
    sink: Sink,
    playing: bool,
    global_settings: GlobalSettings,
    playback_settings: PlaybackSettings,
    music_library: Arc<Mutex<Library>>,
}

impl Default for Jukebox {
    fn default() -> Self {
        let sink = audio::new_sink();
        sink.pause(); // prevents songs auto-playing when added to an empty queue

        Self {
            sink,
            playing: false,
            global_settings: GlobalSettings::default(), // TODO fetch
            playback_settings: PlaybackSettings::default(), // TODO fetch
            music_library: Arc::new(Mutex::new(Library::new())),
        }
    }
}

// Functionality
impl Jukebox {
    fn toggle_play(&mut self) {
        let sink = &self.sink;
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause()
        }
    }

    fn add_song_to_queue(&self, song: Song) -> Result<()> {
        //TODO this autoplays unless the Sink is alreay paused
        let _ = &self
            .sink
            .append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
                song.file_path,
            )?))?);
        println!("added song to queue: {} by {}", song.title, song.artist);
        Ok(())
    }

    // fn read_or_create_config(&mut self, config_path: &str) -> Result<GlobalSettings> {
    //     // TODO check if config file exists, if not create it with defaults, if so read/parse it
    //     if !Path::new(config_path).exists() {
    //         let default_settings = toml::to_string(&GlobalSettings::default())?;
    //         std::fs::write(config_path, default_settings)?;
    //     }
    //     let settings: GlobalSettings = Figment::new().merge(Toml::file(config_path)).extract()?;
    //     Ok(settings)
    // }
}

// UI/Iced
impl Application for Jukebox {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self::default();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Jukebox")
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        match event {
            Message::TogglePlayback => {
                self.toggle_play();
                Command::none()
            }
            Message::AddTestSongToQueue => {
                // TODO remove; has been replaced by PickSong(id)
                let _ = &self
                    .add_song_to_queue(Song {
                        id: 0,
                        title: "test song".to_owned(),
                        artist: "test artist".to_owned(),
                        duration: 60,
                        album_id: None,
                        file_path: "./test.ogg".into(),
                    })
                    .expect("adding song to queue failed");
                Command::none()
            }
            Message::Scan => {
                let library = Arc::clone(&self.music_library);
                let folder_path = "D:/Music";
                Command::perform(
                    async move {
                        let mut lib = library.lock().unwrap();
                        lib.create_songs_from_folder(&folder_path)
                            .map_err(|e| e.to_string())
                    },
                    Message::ScanComplete,
                )
            }
            Message::ScanComplete(result) => {
                match result {
                    Ok(()) => {
                        // let lib = self.music_library.lock().unwrap();
                        // let songs = lib.songs.values().cloned().collect();
                        // self.scan_status =
                        //     format!("Scan complete. Found {} songs.", self.songs.len());
                    }
                    Err(e) => {
                        // self.scan_status = format!("Scan failed: {}", e);
                        format!("Scan failed: {}", e);
                    }
                }
                Command::none()
            }
            Message::PickSong(id) => {
                self.add_song_to_queue(
                    self.music_library
                        .lock()
                        .unwrap()
                        .get_song(id)
                        .unwrap()
                        .clone(),
                )
                .expect("adding song to queue failed");
                Command::none()
            }
            Message::SaveLibrary => {
                let library = Arc::clone(&self.music_library);
                let save_path = &self.global_settings.library_file;
                let _ = library.lock().unwrap().save_to_file(&save_path);
                // Command::perform(
                //     async move {
                //         let lib = library.lock().unwrap();
                //         lib.save_to_file(&save_path)
                //     },
                //     Message::SaveComplete,
                // )
                Command::none()
            }
            Message::SaveComplete(result) => {
                match result {
                    Ok(()) => {
                        // self.scan_status = "Library saved successfully.".to_string();
                    }
                    Err(e) => {
                        // self.scan_status = format!("Save failed: {}", e);
                        format!("Save failed: {}", e);
                    }
                }
                Command::none()
            }
            Message::LoadLibrary => {
                let load_path = self.global_settings.library_file.clone();
                let library = Arc::clone(&self.music_library);
                Command::perform(
                    async move {
                        Library::read_from_file(&load_path)
                            .map(|new_lib| {
                                let mut lib = library.lock().unwrap();
                                *lib = new_lib;
                            })
                            .map_err(|e| e.to_string())
                    },
                    Message::LoadComplete,
                )
            }
            Message::LoadComplete(result) => {
                match result {
                    Ok(()) => {
                        println!("Library loaded successfully.");
                    }
                    Err(e) => {
                        println!("Load failed: {}", e);
                    }
                }
                Command::none()
            }
            Message::PreviousSong => todo!(),
            Message::NextSong => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let play_pause_text = if self.playing { "Pause" } else { "Play" };

        let debug_save_load_buttons = row![
            button("save_to_file").on_press(Message::SaveLibrary),
            button("load_from_file").on_press(Message::LoadLibrary)
        ];

        let song_list = self.music_library.lock().unwrap().songs.iter().fold(
            column![].spacing(5),
            |column, (_id, song)| {
                column.push(
                    button(text(format!(
                        "{} - {} ({})",
                        song.title, song.artist, song.duration
                    )))
                    .on_press(Message::PickSong(song.id)),
                )
            },
        );
        let scan_zone = column![
            button("scan folder").on_press(Message::Scan),
            scrollable(song_list).height(Length::Fill)
        ];
        let controls = row![
            button(play_pause_text).on_press(Message::TogglePlayback),
            button("add test song").on_press(Message::AddTestSongToQueue),
        ];
        let global_layout = column![controls, debug_save_load_buttons, scan_zone];

        container(global_layout)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .into()
    }
}

pub fn main() -> iced::Result {
    // let settings = read_or_create_config("Settings.toml");

    Jukebox::run(Settings::default())
}
