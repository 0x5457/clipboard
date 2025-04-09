use crate::{
    history::History,
    icon::{Icon, IconName},
    style,
};
use gpui::*;
pub struct Workspace {
    pub history: View<History>,
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x252122))
            .size_full()
            .child(Header {})
            .child(self.history.clone())
    }
}

#[derive(IntoElement)]
pub struct Header {}

impl RenderOnce for Header {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .relative()
            .bg(blue())
            .h_7()
            .child(div().rounded_3xl().w_20().h_1().bg(rgb(0xc0c0c0)))
            .child(CloseButton {})
    }
}

#[derive(IntoElement)]
pub struct CloseButton {}

impl RenderOnce for CloseButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .absolute()
            .right_3()
            .justify_center()
            .items_center()
            .cursor_pointer()
            .child(Icon::new(
                IconName::Close,
                Some(style::Size::Medium),
                hsla(0.0, 0.0, 0.75, 1.0),
            ))
    }
}
