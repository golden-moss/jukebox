use std::collections::{HashMap, VecDeque};

use iced::widget::Space;
/// Application is REQUIRED for macros despite being "unused"
use iced::Application;
use iced::{
    alignment,
    widget::{button, column, container, pick_list, row, scrollable, text, text_input},
    Element, Length, Theme,
};
use uuid::Uuid;

use crate::library::Song;
use crate::{Message, UIState};

pub fn centered_text<'a>(string: String) -> Element<'a, Message> {
    container(
        text(string)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    // .width(Length::Fill)
    // .height(Length::Fill)
    .center_y()
    .into()
}

pub fn centered_button<'a>(string: String, message: Message) -> Element<'a, Message> {
    container(button(text(string)).on_press(message))
        .width(Length::Fill)
        // .height(Length::Shrink)
        .center_y()
        .into()
}

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
                    "{} - {} ({:?}) : {}",
                    song.title, song.artist.name, song.duration, is_current
                )))
            },)
    ]
    .into()
}

pub fn library_controls<'a>() -> Element<'a, Message> {
    let layout = column![
        centered_text("library controls".into()),
        button("scan folder").on_press(Message::Scan),
    ];

    container(layout)
        .height(Length::Shrink)
        .width(Length::Fill)
        .center_y()
        .center_x()
        .padding(4)
        .into()
}

pub fn library_song_list<'a>(songs: HashMap<Uuid, Song>) -> Element<'a, Message> {
    scrollable(songs.iter().fold(column![], |column, (id, song)| {
        column.push(centered_button(
            format!(
                "{} - {} ({:?})",
                song.title, song.artist.name, song.duration
            ),
            Message::PickSong(*id),
        ))
    }))
    .height(Length::Fill)
    .into()
}

// pub fn theme_selector<'a>(current_theme: &'a Theme) -> Element<'a, Message> {
//     let choose_theme = column![
//         text("Theme:"),
//         pick_list(Theme::ALL, Some(current_theme), Message::ThemeChanged).width(Length::Fill),
//     ]
//     .spacing(10);
//     container(choose_theme).into()
// }

pub fn change_ui<'a>() -> Element<'a, Message> {
    let button_box = row![
        centered_button(
            "Dont press this one".into(),
            Message::ChangeUI(UIState::Loading)
        ),
        centered_button("Main".into(), Message::ChangeUI(UIState::Main)),
        centered_button("Settings".into(), Message::ChangeUI(UIState::Settings)),
    ]
    .width(Length::Fill);
    container(button_box).into()
}
