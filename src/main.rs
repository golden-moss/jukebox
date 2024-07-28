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
// use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig};
use std::{collections::VecDeque, fs, io::BufReader, sync::Arc, time::Duration};
use ui::{loading_ui, main_ui, settings_ui};

use iced::{executor, Application, Command, Element, Settings, Subscription, Theme};

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

// #[derive(Debug, Clone)]
// enum Theme {
//     Dracula,
// }

#[derive(Debug, Clone)]
enum Message {
    TogglePlayback,
    PreviousSong,
    NextSong,
    AddTestSongToQueue,
    PickSong(u64),
    Scan,
    ScanComplete(Result<(), String>),
    LoadComplete(Result<(), String>),
    ThemeChanged(Theme),
    SaveSettings(GlobalSettings),
    ChangeUI(UIState),
    TickUpdate,
}

#[derive(Debug, Clone)]
enum UIState {
    Loading,
    Main, //current screen
    Settings,
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
    theme: Theme,
    music_library: Arc<Mutex<Library>>,
    playback_queue: Arc<Mutex<VecDeque<(Song, bool)>>>,
    playback_index: usize,
}

impl Default for Jukebox {
    fn default() -> Self {
        Self {
            sink: Arc::new(Mutex::new(None)),
            global_settings: Self::read_or_create_config(),
            playback_settings: PlaybackSettings::default(), // TODO fetch
            ui_state: UIState::Loading,
            theme: Theme::Light,
            music_library: Arc::new(Mutex::new(Library::new())),
            playback_queue: Arc::new(Mutex::new(VecDeque::new())),
            playback_index: 0,
        }
    }
}

// Functionality
impl Jukebox {
    fn toggle_sink_playback(&mut self) {
        if self.sink.lock().as_ref().unwrap().is_paused() {
            self.sink.lock().as_ref().unwrap().play();
        } else {
            self.sink.lock().as_ref().unwrap().pause()
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
        self.replace_sink()?;

        for (_song, current) in self.playback_queue.lock().iter_mut() {
            *current = false;
        }

        if let Some((song, _is_current)) = self.playback_queue.lock().get_mut(self.playback_index) {
            *_is_current = true;

            self.sink
                .lock()
                .as_ref()
                .unwrap()
                .append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
                    &song.file_path,
                )?))?);
            println!("added song: {} by {}", song.title, song.artist);
        }

        Ok(())
    }

    fn replace_sink(&mut self) -> Result<()> {
        self.kill_sink()?;
        self.sink = Arc::new(Mutex::new(Some(audio::new_sink(self.playback_settings))));
        println!("sink created");
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

    fn update_time(&mut self) {
        let time_remaining = self
            .playback_queue
            .lock()
            .get(self.playback_index)
            .unwrap_or(&(Song::default(), false))
            .0
            .duration
            .as_secs()
            - self
                .sink
                .lock()
                .as_ref()
                .unwrap_or(&Sink::new_idle().0)
                .get_pos()
                .as_secs();
        println!("song duration remaining: {:?}", time_remaining);
        if self.sink.lock().is_some() {
            if !self.sink.lock().as_ref().unwrap().is_paused()
                && time_remaining <= Duration::ZERO.as_secs()
            {
                self.next_in_queue();
            }
        }
    }

    fn next_in_queue(&mut self) -> Result<()> {
        const PREVENT_SKIP_BEYOND_QUEUE_LENGTH: usize = 1;
        if self.playback_queue.lock().len() == 0 {
            return Ok(());
        }
        if self.playback_index < self.playback_queue.lock().len() - PREVENT_SKIP_BEYOND_QUEUE_LENGTH
        {
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

    fn scan_and_save(&mut self) -> Result<(), String> {
        //scan
        let _ = self
            .music_library
            .lock()
            .create_songs_from_folder(&self.global_settings.folder_to_scan)
            .map_err(|e| return e.to_string());
        //save
        let save_path = &self.global_settings.library_file;
        self.music_library
            .lock()
            .save_to_file(&save_path)
            .map_err(|e| return format!("SaveLibrary Error: {}", e))
    }

    fn read_or_create_config() -> GlobalSettings {
        let settings = fs::read_to_string("Settings.toml");
        match settings {
            Ok(settings) => toml::from_str(&settings).unwrap_or(GlobalSettings::default()),
            Err(err) => {
                println!("No Settings File: {}", err);
                GlobalSettings::default()
            }
        }
    }
}

// UI/Iced
impl Application for Jukebox {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self::default();
        (
            app.clone(),
            Command::perform(async move { app.load_library() }, Message::LoadComplete),
        )
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Jukebox")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // TODO get key input (handle media keys)
        const TICK_DURATION: f32 = 0.1;

        let time =
            iced::time::every(Duration::from_secs_f32(TICK_DURATION)).map(|_| Message::TickUpdate);

        Subscription::batch([time])
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        match self.ui_state {
            UIState::Loading => match event {
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
            UIState::Main => match event {
                Message::TickUpdate => {
                    self.update_time();
                    Command::none()
                }
                Message::TogglePlayback => {
                    if self.sink.lock().is_none() {
                        let _ = self.play_song_from_queue();
                    } else {
                        self.toggle_sink_playback();
                    }
                    Command::none()
                }
                Message::AddTestSongToQueue => {
                    self.add_song_to_queue_end(Song {
                        id: 0,
                        title: "test song".to_owned(),
                        artist: "test artist".to_owned(),
                        duration: Duration::from_secs(60),
                        album_id: None,
                        file_path: "./test.ogg".into(),
                    })
                    .expect("adding song to queue failed");
                    Command::none()
                }
                Message::Scan => {
                    let mut jb = self.clone();
                    Command::perform(async move { jb.scan_and_save() }, Message::ScanComplete)
                }
                Message::ScanComplete(result) => {
                    match result {
                        Ok(()) => {}
                        Err(e) => {
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
                    let _ = self.prev_in_queue();
                    Command::none()
                }
                Message::NextSong => {
                    let _ = self.next_in_queue();
                    Command::none()
                }
                Message::ThemeChanged(theme) => {
                    self.theme = theme;
                    Command::none()
                }
                Message::ChangeUI(ui_state) => {
                    self.ui_state = ui_state;
                    Command::none()
                }
                _ => Command::none(),
            },
            UIState::Settings => match event {
                Message::SaveSettings(new_settings) => {
                    self.global_settings = new_settings;
                    Command::none()
                }
                Message::ChangeUI(ui_state) => {
                    self.ui_state = ui_state;
                    Command::none()
                }
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self.ui_state {
            UIState::Loading => loading_ui(),
            UIState::Main => main_ui(self.clone()),
            UIState::Settings => settings_ui(self.global_settings.clone()),
        }
    }
}

pub fn main() -> iced::Result {
    Jukebox::run(Settings::default())
}
