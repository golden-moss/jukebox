use gpui::*;
use prelude::*;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::library::{Artist, Song};
use crate::{Jukebox, UIState};

use crate::ui::components::{
    change_ui, library_controls, library_song_list, playback_controls, playback_queue,
};

pub struct MainUI {
    pub jukebox: Jukebox,
    pub focus_handle: FocusHandle,
}

impl FocusableView for MainUI {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainUI {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);

        let (now_playing, _current) = self
            .jukebox
            .playback_queue
            .lock()
            .get(self.jukebox.playback_index)
            .unwrap_or(&(Song::default(), true))
            .clone();
        let queue = self.jukebox.playback_queue.lock().clone();
        let songs = self.jukebox.music_library.lock().songs.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xffddcc))
            .child(change_ui())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(div().flex().child(playback_queue(queue)))
                    .child(
                        div()
                            .flex()
                            .child(library_controls())
                            .child(library_song_list(songs)),
                    ),
            )
            .child(playback_controls(now_playing))
    }
}
