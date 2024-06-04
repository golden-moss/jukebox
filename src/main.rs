// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use anyhow::Result;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::{io::BufReader, path::Path};

use iced::{
    executor,
    widget::{button, container, horizontal_space, row},
    Application, Command, Size, Subscription,
};
use iced::{Element, Length, Settings, Theme};

#[derive(Debug, Serialize, Deserialize)]
struct GlobalSettings {
    folder_to_scan: String, // TODO add ability to scan multiple folders
                            // theme: VisualTheme
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            folder_to_scan: String::from("./"),
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

#[derive(serde::Serialize, Debug, Clone)]
pub enum PlaybackState {
    Play,
    Pause,
    Stop,
}

#[derive(Debug, Clone)]
pub enum Message {
    PlaybackState(PlaybackState),
    AddSongToQueue(String),
}

struct Jukebox {
    sink: Sink,
    global_settings: GlobalSettings,
    playback_settings: PlaybackSettings,
}

impl Default for Jukebox {
    fn default() -> Self {
        let sink = audio::new_sink();

        Self {
            sink,
            global_settings: GlobalSettings::default(), // TODO
            playback_settings: PlaybackSettings::default(), // TODO
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

    fn add_song_to_queue(&self, song: &str) -> Result<()> {
        let _ = &self
            .sink
            .append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
                song,
            )?))?);
        println!("added song to queue: {}", song);
        Ok(())
    }

    fn read_or_create_config(&mut self, config_path: &str) -> Result<GlobalSettings> {
        // TODO check if config file exists, if not create it with defaults, if so read/parse it
        if !Path::new(config_path).exists() {
            let default_settings = toml::to_string(&GlobalSettings::default())?;
            std::fs::write(config_path, default_settings)?;
        }
        let settings: GlobalSettings = Figment::new().merge(Toml::file(config_path)).extract()?;
        Ok(settings)
    }
}

// UI/Iced
impl Application for Jukebox {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let app = Self::default();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Jukebox")
    }

    fn update(&mut self, event: Message) -> iced::Command<Self::Message> {
        match event {
            Message::PlaybackState(state) => match state {
                PlaybackState::Play => {
                    let _ = &self
                        .add_song_to_queue("test.flac")
                        .expect("adding song to queue failed");
                    println!("start playback");
                    println!("{:?}", self.sink.len());
                    self.sink.play();
                }
                PlaybackState::Pause => {
                    self.sink.pause();
                }
                PlaybackState::Stop => {
                    self.sink.stop();
                }
            },
            Message::AddSongToQueue(song) => {
                let _ = &self
                    .add_song_to_queue(&song)
                    .expect("adding song to queue failed");
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let controls = row![]
            .push(button("Play").on_press(Message::PlaybackState(PlaybackState::Play)))
            .push(horizontal_space())
            .push(button("Stop").on_press(Message::PlaybackState(PlaybackState::Stop)))
            .push(horizontal_space())
            .push(button("add song").on_press(Message::AddSongToQueue("TODO".to_string())));

        container(controls).height(Length::Shrink).center_y().into()
    }
}

pub fn main() -> iced::Result {
    // let settings = read_or_create_config("Settings.toml");

    Jukebox::run(Settings::default())
}
