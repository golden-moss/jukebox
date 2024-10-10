use gpui::*;
use prelude::*;

// use gpui::Styled;
// use gpui::{
//     div, rgb, ClickEvent, ElementId, InteractiveElement, IntoElement, RenderOnce, SharedString,
//     WindowContext,
// };

use crate::ui::theme::*;

pub fn button(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
    Button {
        id: id.into(),
        label: label.into(),
        on_click: None,
    }
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Button {
            id: id.into(),
            label,
            on_click: None,
        }
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .cursor_pointer()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, cx| (on_click)(evt, cx))
            })
            .child(self.label)
    }
}
