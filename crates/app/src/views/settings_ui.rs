use gpui::*;
use prelude::*;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::library::{Artist, Song};
use crate::{Jukebox, UIState};

use crate::ui::components::{
    change_ui, h4, library_controls, library_song_list, playback_controls, playback_queue, text,
};

pub struct SettingsUI {
    pub jukebox: Jukebox,
    pub focus_handle: FocusHandle,
}

impl FocusableView for SettingsUI {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SettingsUI {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);

        // TODO generate settings screen based on GlobalSettings struct
        let mut settings = self.jukebox.global_settings.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(change_ui())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(h4("Folder to scan:"))
                    .child(text("text input field")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(h4("Library File:"))
                    .child(text("text input field")),
            )
    }
}
