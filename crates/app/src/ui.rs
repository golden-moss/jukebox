use gpui::{div, Div, IntoElement, Styled};

pub(crate) mod components;
pub(crate) mod theme;

/// Creates a Div with the width and height set to 100%.
pub fn root(/*cx: &mut ViewContext<Self>*/) -> Div {
    div().size_full()
}
