use iced::{
    alignment,
    widget::{button, column, container, pick_list, text},
    Element, Length, Theme,
};

use crate::Message;

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

pub fn theme_selector<'a>(current_theme: &'a Theme) -> Element<'a, Message> {
    let choose_theme = column![
        text("Theme:"),
        pick_list(Theme::ALL, Some(current_theme), Message::ThemeChanged).width(Length::Fill),
    ]
    .spacing(10);
    container(choose_theme).into()
}
