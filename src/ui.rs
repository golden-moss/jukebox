use components::{
    centered_button, centered_text, change_ui, library_controls, library_song_list,
    playback_controls, playback_queue,
};
use iced::widget::text_input;
/// REQUIRED for macros despite being "unused"
use iced::Application;
use iced::{
    widget::{column, container, row, scrollable, text},
    Alignment, Element, Length,
};

use crate::library::Song;
use crate::Message;
use crate::{GlobalSettings, Jukebox};

mod components;
mod theme;

// pub fn ui<'a>() -> Element<'a, Message> {}

pub fn loading_ui<'a>() -> Element<'a, Message> {
    container(row![centered_text("Loading...".into())])
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn main_ui<'a>(jb: Jukebox) -> Element<'a, Message> {
    let (now_playing, _current) = jb
        .playback_queue
        .lock()
        .get(jb.playback_index)
        .unwrap_or(&(Song::default(), true))
        .clone();

    let navbar = change_ui();

    let left_col =
        column![playback_queue(jb.playback_queue.lock().clone())].align_items(Alignment::Start);
    let right_col = column![
        library_controls(),
        // theme_selector(&jb.theme),
        library_song_list(jb.music_library.lock().songs.clone())
    ]
    .align_items(Alignment::Start);

    let global_layout = column![row![left_col, right_col], playback_controls(now_playing)];

    container(column![navbar, global_layout])
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
}

pub fn settings_ui<'a>(settings: GlobalSettings) -> Element<'a, Message> {
    // TODO convert to macro later so that it does not need to be manually updated with every change to GlobalSettings
    let mut new_settings = settings;

    let navbar = change_ui();

    let items = scrollable(
        column![]
            .push(row![
                text("Folder to scan:"),
                text_input("settings.folder_to_scan", &new_settings.folder_to_scan)
                    // .on_input(Message::SaveSettings(new_settings))
                    .padding(10)
                    .size(20),
            ])
            .push(row![
                text("Library File:"),
                text_input("settings.library_file", &new_settings.library_file)
                    // .on_input(Message::SaveSettings(new_settings))
                    .padding(10)
                    .size(20),
            ]), // .push(centered_button(
                // "save settings".into(),
                // Message::SaveSettings(new_settings),
                // )),
    )
    .height(Length::Fill);

    container(column![navbar, items]).into()
}
