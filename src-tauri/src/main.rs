// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lofty::file::TaggedFileExt;
use lofty::read_from_path;

use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::{fmt, io::BufReader, path::Path};
use thiserror::Error;

#[derive(Default, Debug, Serialize, Deserialize)]
struct GlobalSettings {
    folder_to_scan: String, // TODO add ability to scan multiple folders
    volume: f32,            // lets leave this at 1.0 for now
                            // theme: VisualTheme
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("global settings error")]
    SettingsError(String),
}

struct Jukebox(Sink);

#[derive(serde::Serialize)]
pub enum PlaybackState {
    Play,
    Pause,
    Stop,
}

// Functions

fn deal_with_config() -> Result<(), MyError> {
    // TODO check if config file exists, if not create it with defaults, if so read/parse it
    // let config = ron::from_str(std::fs::File::open(path::Path::new("./config.ron")).unwrap()).unwrap();
    Ok(())

    // let x: GlobalSettings = ron::from_str("(folder_to_scan: ./, volume: 1.0)").unwrap();
}

// Tauri Commands

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn read_tags(path: &str) -> String {
    println!("file: {}", path);
    // This will guess the format from the extension
    let tagged_file = read_from_path(path).unwrap();
    format!("{:?}", tagged_file.file_type())
}

#[tauri::command]
fn toggle_playback(state: tauri::State<Jukebox>) -> PlaybackState {
    // TODO currently this requires 2 clicks to start playback, since the frontend does know the state ahead of time, so the first click will return Pause and not actually start the playback until the second click, when it is toggled to Play.
    let jb = &state.0;
    jb.append(
        rodio::Decoder::new(BufReader::new(std::fs::File::open("../test.flac").unwrap())).unwrap(),
    );
    if jb.is_paused() {
        jb.play();
        PlaybackState::Play
    } else {
        jb.pause();
        PlaybackState::Pause
    }
}

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    tauri::Builder::default()
        .manage(Jukebox(sink))
        .invoke_handler(tauri::generate_handler![greet, read_tags, toggle_playback])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
