use std::collections::{HashMap, VecDeque};

use iced::widget::Space;
use iced::{
    alignment,
    widget::{button, column, container, pick_list, row, scrollable, text, text_input},
    Element, Length, Theme,
};
/// Application is REQUIRED for macros despite being "unused"
use iced::{Alignment, Application};
use uuid::Uuid;

use crate::library::{Artist, Song};
use crate::{Message, UIState};

pub fn centered_title<'a>(string: String) -> Element<'a, Message> {
    container(text_h1(string))
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

/// Text
/// Body font size: 16px
/// Line height: 1.6 * font size
pub fn text_h1<'a>(string: String) -> Element<'a, Message> {
    text(string).size(48).line_height(1.6).into()
}
pub fn text_h2<'a>(string: String) -> Element<'a, Message> {
    text(string).size(42).line_height(1.6).into()
}
pub fn text_h3<'a>(string: String) -> Element<'a, Message> {
    text(string).size(38).line_height(1.6).into()
}
pub fn text_h4<'a>(string: String) -> Element<'a, Message> {
    text(string).size(34).line_height(1.6).into()
}
pub fn text_h5<'a>(string: String) -> Element<'a, Message> {
    text(string).size(28).line_height(1.6).into()
}
pub fn text_h6<'a>(string: String) -> Element<'a, Message> {
    text(string).size(22).line_height(1.6).into()
}
pub fn text_p<'a>(string: String) -> Element<'a, Message> {
    text(string).size(16).line_height(1.6).into()
}

pub fn playback_controls<'a>(now_playing: Song) -> Element<'a, Message> {
    column![
        text_h4(now_playing.title),
        row![
            text_p(now_playing.album.unwrap_or_default().title),
            text_p(
                now_playing
                    .artists
                    .first()
                    .unwrap_or(&Artist::default())
                    .name
                    .clone()
            )
        ]
        .spacing(8),
        row![
            button("previous song").on_press(Message::PreviousSong),
            button("play or pause").on_press(Message::TogglePlayback),
            button("next song").on_press(Message::NextSong),
        ]
        .spacing(2)
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(2)
    .padding(8)
    .into()
}

pub fn playback_queue<'a>(queue: VecDeque<(Song, bool)>) -> Element<'a, Message> {
    column![
        centered_title("Queue".into()),
        queue
            .iter()
            .fold(column![].spacing(0.25), |column, (song, is_current)| {
                column.push(text_p(format!(
                    "{} - {} ({:?}) : {}",
                    song.title,
                    song.artists.first().unwrap_or(&Artist::default()).name,
                    song.duration,
                    is_current
                )))
            },)
    ]
    .padding(12)
    .max_width(300)
    .spacing(4)
    .into()
}

pub fn library_controls<'a>() -> Element<'a, Message> {
    let layout = column![
        centered_title("library controls".into()),
        row![
            button("scan folder").on_press(Message::Scan),
            button("add test song").on_press(Message::AddTestSongToQueue),
        ]
        .spacing(2),
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
    container(scrollable(songs.iter().fold(
        column![],
        |column, (id, song)| {
            column.push(centered_button(
                format!(
                    "{} - {} ({:?})",
                    song.title,
                    song.artists.first().unwrap_or(&Artist::default()).name,
                    song.duration
                ),
                Message::PickSong(*id),
            ))
        },
    )))
    .height(Length::Fill)
    .padding(12)
    .max_width(700)
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
