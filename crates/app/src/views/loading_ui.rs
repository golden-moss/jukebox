use gpui::*;
use prelude::*;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::library::{Artist, Song};
use crate::{Jukebox, UIState};

use crate::ui::components::centered_title;

pub struct LoadingUI {
    pub focus_handle: FocusHandle,
}

impl LoadingUI {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for LoadingUI {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LoadingUI {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x333333))
            .child(centered_title("Loading..."))
    }
}
