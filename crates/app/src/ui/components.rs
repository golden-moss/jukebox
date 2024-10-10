use gpui::*;
use prelude::*;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::library::{Artist, Song};
use crate::UIState;
use button::button;

pub(crate) mod button;

pub fn centered_title(string: impl Into<SharedString>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .size_full()
        .text_3xl()
        .child(string.into())
}

pub fn centered_button(string: impl Into<SharedString>) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .child(button("new-button", string))
}

// Text
// Body font size: 16px
// Line height: 1.6 * font size
pub fn h1(string: impl Into<SharedString>) -> impl IntoElement {
    div().text_3xl().child(string.into())
}
pub fn h2(string: impl Into<SharedString>) -> impl IntoElement {
    div().text_2xl().child(string.into())
}
pub fn h3(string: impl Into<SharedString>) -> impl IntoElement {
    div().text_xl().child(string.into())
}
pub fn h4(string: impl Into<SharedString>) -> impl IntoElement {
    div().text_lg().child(string.into())
}
pub fn text(string: impl Into<SharedString>) -> impl IntoElement {
    div().child(string.into())
}

pub fn playback_controls(now_playing: Song) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .child(h4(now_playing.title))
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(text(now_playing.album.unwrap_or_default().title)),
        )
        .child(text(
            now_playing
                .artists
                .first()
                .unwrap_or(&Artist::default())
                .name
                .clone(),
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .gap_1()
                .child(centered_button("previous song"))
                .child(centered_button("play or pause"))
                .child(centered_button("next song")),
        )
}

pub fn playback_queue(queue: VecDeque<(Song, bool)>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .child(centered_title("Queue"))
        // TODO implement scrollable behavior (maybe)
        .child(
            div()
                .flex()
                .flex_col()
                .children(queue.iter().map(|(song, is_current)| {
                    text(format!(
                        "{} - {} ({:?}) : {}",
                        song.title,
                        song.artists.first().unwrap_or(&Artist::default()).name,
                        song.duration,
                        is_current
                    ))
                })),
        )
}

pub fn library_controls() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .justify_center()
        .items_center()
        .size_full()
        .child(centered_title("library controls"))
        .child(
            div()
                .flex()
                .flex_row()
                .gap_1()
                .child(button("scan", "scan folder")) // .on_press(Message::Scan),
                .child(button("addtest", "add test song")), // .on_press(Message::AddTestSongToQueue),
        )
}

pub fn library_song_list(songs: HashMap<Uuid, Song>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        // TODO implement scrollable behavior (maybe)
        .child(
            div()
                .flex()
                .flex_col()
                .children(songs.iter().map(|(id, song)| {
                    button(
                        *id,
                        format!(
                            "{} - {} ({:?})",
                            song.title,
                            song.artists.first().unwrap_or(&Artist::default()).name,
                            song.duration,
                        ),
                    )
                })),
        )
}

pub fn change_ui() -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .content_center()
        // .size_2()
        .child(button("ui_to_loading", "Dont press this one"))
        .child(button("ui_to_main", "Main"))
        .child(button("ui_to_settings", "Settings"))
}
