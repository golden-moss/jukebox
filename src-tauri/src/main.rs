// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lofty::read_from_path;
use lofty::{error::LoftyError, file::TaggedFileExt};

use figment::{
    providers::{Format, Toml},
    Figment,
};
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::{io::BufReader, path::Path};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct GlobalSettings {
    folder_to_scan: String, // TODO add ability to scan multiple folders
                            // theme: VisualTheme
}

struct PlaybackSettings {
    volume: f32, // lets leave this at 1.0 for now
    speed: f32,  // lets leave this at 1.0 for now
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            folder_to_scan: String::from("./"),
        }
    }
}

#[derive(Error, Debug)]
pub enum JbError {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("global settings error")]
    SettingsError(String),
    #[error("audio file read error")]
    AudioFileError(#[from] rodio::decoder::DecoderError),
    #[error("figment error")]
    SettingsParsingError(#[from] figment::Error),
    #[error("lofty error")]
    AudioTagError(#[from] LoftyError),
}

// we must manually implement serde::Serialize
impl serde::Serialize for JbError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct Jukebox(Sink);
// struct PlayQueue

#[derive(serde::Serialize)]
pub enum PlaybackState {
    Play,
    Pause,
    Stop,
}

// Tauri Commands

#[tauri::command]
fn read_tags(path: &str) -> Result<String, JbError> {
    println!("file: {}", path);
    // This will guess the format from the extension
    let tagged_file = read_from_path(path)?;
    Ok(format!("{:?}", tagged_file.file_type()))
}

#[tauri::command]
fn get_global_settings() -> Result<GlobalSettings, JbError> {
    Ok(GlobalSettings::default())
}

#[tauri::command]
fn toggle_playback(state: tauri::State<Jukebox>) -> Result<PlaybackState, JbError> {
    let jb = &state.0;
    add_song_to_queue(&jb)?;
    if jb.is_paused() {
        jb.play();
        Ok(PlaybackState::Play)
    } else {
        jb.pause();
        Ok(PlaybackState::Pause)
    }
}

#[tauri::command]
fn stop_playback(state: tauri::State<Jukebox>) -> Result<PlaybackState, JbError> {
    let jb = &state.0;
    jb.stop();
    Ok(PlaybackState::Stop)
}

// Functions

fn read_or_create_config(config_path: &str) -> Result<GlobalSettings, JbError> {
    // TODO check if config file exists, if not create it with defaults, if so read/parse it
    if !Path::new(config_path).exists() {
        let default_settings = toml::to_string(&GlobalSettings::default()).unwrap();
        std::fs::write(config_path, default_settings).unwrap();
    }
    let settings: GlobalSettings = Figment::new().merge(Toml::file(config_path)).extract()?;
    Ok(settings)
}

fn add_song_to_queue(jb: &Sink) -> Result<(), JbError> {
    jb.append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
        "../test.flac",
    )?))?);
    Ok(())
}

fn main() {
    // Rodio sink, plays the audio
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let settings = read_or_create_config("Settings.toml");

    tauri::Builder::default()
        .manage(Jukebox(sink))
        .manage(settings)
        .invoke_handler(tauri::generate_handler![
            read_tags,
            toggle_playback,
            get_global_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
