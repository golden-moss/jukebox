use std::collections::{HashMap, VecDeque};

use components::{centered_text, song_button};
use iced::{
    widget::{button, column, container, row, scrollable, text},
    Application, Element, Length,
};

use crate::{library::Song, Message};

pub mod components;

// pub fn ui<'a>() -> Element<'a, Message> {}
// pub fn ui<'a>() -> Element<'a, Message> {}

pub fn playback_controls<'a>() -> Element<'a, Message> {
    row![
        button("previous song").on_press(Message::PreviousSong),
        button("play or pause").on_press(Message::TogglePlayback),
        button("next song").on_press(Message::NextSong),
        button("add test song").on_press(Message::AddTestSongToQueue),
    ]
    .spacing(2)
    .into()
}

pub fn playback_queue_display<'a>(queue: VecDeque<(Song, bool)>) -> Element<'a, Message> {
    column![
        centered_text("Queue".into()),
        queue
            .iter()
            .fold(column![].spacing(0.25), |column, (song, is_current)| {
                column.push(text(format!(
                    "{} - {} ({}) : {}",
                    song.title, song.artist, song.duration, is_current
                )))
            },)
    ]
    .into()
}

pub fn library_controls<'a>() -> Element<'a, Message> {
    let save_load_buttons = row![
        button("save_to_file").on_press(Message::SaveLibrary),
        button("load_from_file").on_press(Message::LoadLibrary)
    ];

    let layout = column![
        centered_text("library controls".into()),
        save_load_buttons,
        button("scan folder").on_press(Message::Scan)
    ];

    container(layout)
        .height(Length::Shrink)
        .width(Length::Fill)
        .center_y()
        .center_x()
        .padding(4)
        .into()
}

pub fn library_song_list<'a>(songs: HashMap<u64, Song>) -> Element<'a, Message> {
    scrollable(songs.iter().fold(column![], |column, (_id, song)| {
        column.push(song_button(
            format!("{} - {} ({})", song.title, song.artist, song.duration),
            Message::PickSong(song.id),
        ))
    }))
    .height(Length::Fill)
    .into()
}
