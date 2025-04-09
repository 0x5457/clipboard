use gpui::*;
use prelude::FluentBuilder as _;

use crate::style;

pub enum IconName {
    Close,
}

impl IconName {
    pub fn path(self) -> SharedString {
        match self {
            IconName::Close => "icons/close.svg",
        }
        .into()
    }
}

#[derive(IntoElement)]
pub struct Icon {
    base: Svg,
    path: SharedString,
    text_color: Hsla,
    size: Option<style::Size>,
}

impl Icon {
    pub fn new(i: IconName, size: Option<style::Size>, text_color: Hsla) -> Self {
        Self {
            base: svg().flex_none().size_4(),
            path: i.path(),
            text_color,
            size,
        }
    }
}

impl RenderOnce for Icon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let text_color = self.text_color;
        self.base
            .text_color(text_color)
            .when_some(self.size, |this, size| match size {
                style::Size::Custom(px) => this.size(px),
                style::Size::XSmall => this.size_3(),
                style::Size::Small => this.size_3p5(),
                style::Size::Medium => this.size_4(),
                style::Size::Large => this.size_6(),
            })
            .path(self.path)
    }
}
