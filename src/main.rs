// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod library;
mod ui;

use anyhow::Result;
use library::{Library, Song};
use parking_lot::Mutex;
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, io::BufReader, sync::Arc};
use ui::{
    library_controls, library_song_list, loading_ui, playback_controls, playback_queue_display,
};

use iced::{
    executor,
    widget::{column, container, row},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone)]
enum UIState {
    Loading,
    Main, //current screen
          // Artist(id) // not sure how to best implement
          // Album(id) // not sure how to best implement
          // Song?(id) // not sure how to best implement
}

#[derive(Clone)]
struct Jukebox {
    sink: Arc<Mutex<Option<Sink>>>,
    global_settings: GlobalSettings,
    playback_settings: PlaybackSettings,
    ui_state: UIState,
    music_library: Arc<Mutex<Library>>,
    playback_queue: Arc<Mutex<VecDeque<(Song, bool)>>>,
    playback_index: usize,
}

impl Default for Jukebox {
    fn default() -> Self {
        Self {
            sink: Arc::new(Mutex::new(None)),
            global_settings: GlobalSettings::default(), // TODO fetch
            playback_settings: PlaybackSettings::default(), // TODO fetch
            ui_state: UIState::Loading,
            music_library: Arc::new(Mutex::new(Library::new())),
            playback_queue: Arc::new(Mutex::new(VecDeque::new())),
            playback_index: 0,
        }
    }
}

// Functionality
impl Jukebox {
    fn toggle_sink_playback(&mut self) {
        if let sink = &self.sink.lock().as_ref().unwrap() {
            if sink.is_paused() {
                sink.play();
            } else {
                sink.pause()
            }
        }
    }

    fn reorder_song_in_queue(&self, new_pos_in_queue: usize) -> Result<()> {
        todo!()
    }

    fn add_song_to_queue_end(&self, song: Song) -> Result<()> {
        self.playback_queue.lock().push_back((song, false));
        Ok(())
    }

    fn add_song_to_queue_start(&self, song: Song) -> Result<()> {
        self.playback_queue.lock().push_front((song, false));
        Ok(())
    }

    fn play_song_from_queue(&mut self) -> Result<()> {
        let _ = &self.replace_sink()?;

        if let Some((song, mut _is_current)) = self.playback_queue.lock().get(self.playback_index) {
            _is_current = true;

            if let sink = &self.sink.lock().as_ref().unwrap() {
                sink.append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
                    &song.file_path,
                )?))?);
                println!(
                    "added song to current sink: {} by {}",
                    song.title, song.artist
                );
            }
        }

        Ok(())
    }

    fn replace_sink(&mut self) -> Result<()> {
        self.kill_sink()?;
        self.sink = Arc::new(Mutex::new(Some(audio::new_sink(self.playback_settings))));
        Ok(())
    }

    fn kill_sink(&mut self) -> Result<()> {
        if self.sink.lock().as_ref().is_some() {
            self.sink = Arc::new(Mutex::new(None));
            println!("sink killed");
        }
        Ok(())
    }

    // fn stop_current_playback(&mut self) -> Result<()> {}

    fn next_in_queue(&mut self) -> Result<()> {
        if self.playback_index < self.playback_queue.lock().len() {
            self.playback_index += 1;
        }
        self.play_song_from_queue()?;
        Ok(())
    }

    fn prev_in_queue(&mut self) -> Result<()> {
        if self.playback_index > 0 {
            self.playback_index -= 1;
        }
        self.play_song_from_queue()?;
        Ok(())
    }

    fn load_library(&self) -> Result<(), String> {
        let load_path = self.global_settings.library_file.clone();
        let library = Arc::clone(&self.music_library);
        Library::read_from_file(&load_path)
            .map(|new_lib| {
                let mut lib = library.lock();
                *lib = new_lib;
            })
            .map_err(|e| e.to_string())
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

    fn new(_flags: () /*, settings: GlobalSettings*/) -> (Self, Command<Message>) {
        let app = Self::default();
        (
            app.clone(),
            Command::perform(async move { app.load_library() }, Message::LoadComplete),
        )
    }

    fn title(&self) -> String {
        String::from("Jukebox")
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        match self.ui_state {
            UIState::Loading => match event {
                Message::LoadLibrary => {
                    let load_path = self.global_settings.library_file.clone();
                    let library = Arc::clone(&self.music_library);
                    Command::perform(
                        async move {
                            Library::read_from_file(&load_path)
                                .map(|new_lib| {
                                    let mut lib = library.lock();
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
                            self.ui_state = UIState::Main
                        }
                        Err(e) => {
                            println!("Load failed: {}", e);
                            self.ui_state = UIState::Main
                        }
                    }
                    Command::none()
                }
                _ => Command::none(),
            },
            UIState::Main => {
                match event {
                    Message::TogglePlayback => {
                        if self.sink.lock().is_none() {
                            self.play_song_from_queue();
                        } else {
                            self.toggle_sink_playback();
                        }
                        Command::none()
                    }
                    Message::AddTestSongToQueue => {
                        let _ = &self
                            .add_song_to_queue_end(Song {
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
                                let mut lib = library.lock();
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
                        self.add_song_to_queue_end(
                            self.music_library.lock().get_song(id).unwrap().clone(),
                        )
                        .expect("adding song to queue failed");
                        Command::none()
                    }
                    Message::SaveLibrary => {
                        let library = Arc::clone(&self.music_library);
                        let save_path = &self.global_settings.library_file;
                        let _ = library.lock().save_to_file(&save_path);
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
                                        let mut lib = library.lock();
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
                    Message::PreviousSong => {
                        self.prev_in_queue();
                        Command::none()
                    }
                    Message::NextSong => {
                        self.next_in_queue();
                        Command::none()
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.ui_state {
            UIState::Loading => loading_ui(),
            UIState::Main => {
                let left_col = column![
                    playback_controls(),
                    playback_queue_display(self.playback_queue.lock().clone())
                ]
                .align_items(Alignment::Start);
                let right_col = column![
                    library_controls(),
                    library_song_list(self.music_library.lock().songs.clone())
                ]
                .align_items(Alignment::Start);

                // this should be a row of columns.
                // let global_layout = row![left_col, right_col];
                let global_layout = row![left_col, right_col];

                container(global_layout)
                    .height(Length::Shrink)
                    .width(Length::Shrink)
                    // .center_y()
                    // .center_x()
                    .into()
            }
        }
    }
}

pub fn main() -> iced::Result {
    // let settings = read_or_create_config("Settings.toml");

    Jukebox::run(Settings::default())
}
