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
use state::InitCell;
use std::fmt::Debug;
use std::sync::RwLock;
use std::{io::BufReader, path::Path};
use thiserror::Error;

use cushy::value::{Destination, Dynamic, IntoReader, Source};
use cushy::widget::MakeWidget;
use cushy::widgets::button::ButtonKind;
use cushy::Run;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    folder_to_scan: String, // TODO add ability to scan multiple folders
                            // theme: VisualTheme
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaybackSettings {
    volume: f32, // lets leave this at 1.0 for now
    speed: f32,  // lets leave this at 1.0 for now
}

impl Default for Settings {
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

// // we must manually implement serde::Serialize
impl serde::Serialize for JbError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct Jukebox {
    sink: Sink,
    state: PlaybackState,
}
// struct PlayQueue

impl Default for Jukebox {
    fn default() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        Self {
            sink,
            state: PlaybackState::Stop,
        }
    }
}

#[derive(serde::Serialize)]
pub enum PlaybackState {
    Play,
    Pause,
    Stop,
}

static JUKEBOX: InitCell<RwLock<Jukebox>> = InitCell::new();
static GLOBAL_SETTINGS: InitCell<RwLock<Settings>> = InitCell::new();

fn read_tags(path: &str) -> Result<String, JbError> {
    println!("file: {}", path);
    // This will guess the format from the extension
    let tagged_file = read_from_path(path)?;
    Ok(format!("{:?}", tagged_file.file_type()))
}

fn get_global_settings() -> Result<Settings, JbError> {
    Ok(Settings::default())
}

fn toggle_playback() -> Result<PlaybackState, JbError> {
    let mut_jb = JUKEBOX.get().write().unwrap();
    add_song_to_queue(&mut_jb.sink)?;
    // TODO this it takes 2 clicks to start the first time because the default state is "stopped" not paused
    if mut_jb.sink.is_paused() {
        mut_jb.sink.play();
        println!("playing");
        Ok(PlaybackState::Play)
    } else {
        mut_jb.sink.pause();
        println!("paused");
        Ok(PlaybackState::Pause)
    }
}

fn stop_playback() -> Result<PlaybackState, JbError> {
    let mut_jb = JUKEBOX.get().write().unwrap();
    mut_jb.sink.stop();
    Ok(PlaybackState::Stop)
}

fn read_or_create_config(config_path: &str) -> Result<Settings, JbError> {
    // TODO check if config file exists, if not create it with defaults, if so read/parse it
    if !Path::new(config_path).exists() {
        let default_settings = toml::to_string(&Settings::default()).unwrap();
        std::fs::write(config_path, default_settings).unwrap();
    }
    let settings: Settings = Figment::new().merge(Toml::file(config_path)).extract()?;
    Ok(settings)
}

fn add_song_to_queue(jb: &Sink) -> Result<(), JbError> {
    jb.append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
        "test.flac",
    )?))?);
    Ok(())
}

fn main() -> cushy::Result {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let settings = read_or_create_config("Settings.toml").unwrap();

    let jb = Jukebox::default();
    // let jb = Dynamic::new(PlaybackState::Play);
    JUKEBOX.set(RwLock::new(jb));
    GLOBAL_SETTINGS.set(RwLock::new(settings));

    // jb.to_label().into_button().on_click(move |sink| )

    // Create a dynamic usize.
    let count = Dynamic::new(0_isize);

    // Create a new label displaying `count`
    count
        .to_label()
        // Use the label as the contents of a button
        .into_button()
        // Set the `on_click` callback to a closure that increments the counter.
        .on_click(move |_| {
            toggle_playback().unwrap();
            count.set(count.get() + 1)
        })
        // Run the application
        .run()
}

// fn main() -> cushy::Result {
//     let clicked_label = Dynamic::new(String::from("Click a Button"));
//     let default_is_outline = Dynamic::new(false);
//     let default_button_style = default_is_outline.map_each(|is_outline| {
//         if *is_outline {
//             ButtonKind::Outline
//         } else {
//             ButtonKind::Solid
//         }
//     });

//     clicked_label
//         .clone()
//         .and(
//             "Normal Button"
//                 .into_button()
//                 .on_click(
//                     clicked_label.with_clone(|label| {
//                         move |_| label.set(String::from("Clicked Normal Button"))
//                     }),
//                 )
//                 .and(
//                     "Outline Button"
//                         .into_button()
//                         .on_click(clicked_label.with_clone(|label| {
//                             move |_| label.set(String::from("Clicked Outline Button"))
//                         }))
//                         .kind(ButtonKind::Outline),
//                 )
//                 .and(
//                     "Transparent Button"
//                         .into_button()
//                         .on_click(clicked_label.with_clone(|label| {
//                             move |_| label.set(String::from("Clicked Transparent Button"))
//                         }))
//                         .kind(ButtonKind::Transparent),
//                 )
//                 .and(
//                     "Default Button"
//                         .into_button()
//                         .on_click(clicked_label.with_clone(|label| {
//                             move |_| label.set(String::from("Clicked Default Button"))
//                         }))
//                         .kind(default_button_style)
//                         .into_default(),
//                 )
//                 .and("Set Default to Outline".into_checkbox(default_is_outline))
//                 .into_columns(),
//         )
//         .into_rows()
//         .centered()
//         .run()
// }
