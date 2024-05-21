// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Ok, Result};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use rodio::{OutputStreamHandle, Sink, Source};
use serde::{Deserialize, Serialize};
use state::InitCell;
use std::fs::File;
use std::ops::DerefMut;
use std::sync::RwLock;
use std::{collections::VecDeque, fmt::Debug};
use std::{io::BufReader, path::Path};

use cushy::value::{Destination, Dynamic, IntoReader, Source as CSource};
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

struct Jukebox {
    out_handle: OutputStreamHandle,
    state: PlaybackState,
    now_playing: Option<Sink>,
    queue: VecDeque<Sink>,
}

impl Default for Jukebox {
    fn default() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        // let sink = Sink::try_new(&handle).unwrap();
        Self {
            out_handle: handle,
            state: PlaybackState::Stop,
            now_playing: None,
            queue: VecDeque::new(),
        }
    }
}

#[derive(serde::Serialize)]
pub enum PlaybackState {
    Play,
    Pause,
    Stop,
}

impl PlaybackState {
    fn change_state(&mut self, state: PlaybackState) -> Self {
        state
    }
}

static JUKEBOX: InitCell<RwLock<Jukebox>> = InitCell::new();
static GLOBAL_SETTINGS: InitCell<RwLock<Settings>> = InitCell::new();

fn read_tags(path: &str) -> Result<String> {
    println!("file: {}", path);
    // This will guess the format from the extension
    let tagged_file = read_from_path(path)?;
    Ok(format!("{:?}", tagged_file.file_type()))
}

fn toggle_playback() -> Result<()> {
    let mut mut_jb = JUKEBOX.get().write().unwrap();
    // let sink = Sink::try_new(&mut_jb.out_handle).unwrap();
    add_song_to_queue()?;
    if mut_jb.now_playing.is_none() {
        mut_jb.now_playing = mut_jb.queue.pop_front();
        todo!("need to check to make sure now_playing is Some() since queue might return None")
    }

    // TODO this it takes 2 clicks to start the first time because the default state is "stopped" not paused
    match mut_jb.state {
        PlaybackState::Play => {
            mut_jb
                .now_playing
                .as_ref()
                .context("failed to toggle playback")?
                .pause();
            mut_jb.state.change_state(PlaybackState::Pause);
            Ok(())
        }
        PlaybackState::Pause => {
            mut_jb
                .now_playing
                .as_ref()
                .context("failed to toggle playback")?
                .play();
            mut_jb.state.change_state(PlaybackState::Play);
            Ok(())
        }
        PlaybackState::Stop => todo!(),
    }
    // if mut_jb.state {
    //     mut_jb.now_playing.unwrap().play();
    //     // mut_jb.deref_mut().sink.detach();
    //     println!("playing");
    //     Ok(())
    // } else {
    //     mut_jb.now_playing.unwrap().pause();
    //     println!("paused");
    //     Ok(())
    // }
}

// fn stop_playback() -> Result<PlaybackState> {
//     let mut_jb = JUKEBOX.get().write().unwrap();
//     mut_jb.out_handle.stop();
//     Ok(PlaybackState::Stop)
// }

fn read_or_create_config(config_path: &str) -> Result<Settings> {
    // TODO check if config file exists, if not create it with defaults, if so read/parse it
    if !Path::new(config_path).exists() {
        let default_settings = toml::to_string(&Settings::default()).unwrap();
        std::fs::write(config_path, default_settings).unwrap();
    }
    let settings: Settings = Figment::new().merge(Toml::file(config_path)).extract()?;
    Ok(settings)
}

fn add_song_to_queue() -> Result<()> {
    // TODO take song as arg
    let mut mut_jb = JUKEBOX.get().write().unwrap();
    let sink = Sink::try_new(&mut_jb.out_handle).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
        "test.flac",
    )?))?);
    mut_jb.queue.push_back(sink);
    Ok(())
}

fn main() -> cushy::Result {
    let settings = read_or_create_config("Settings.toml").unwrap();

    let jb = Jukebox::default();
    JUKEBOX.set(RwLock::new(jb));
    GLOBAL_SETTINGS.set(RwLock::new(settings));

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
        .centered()
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
