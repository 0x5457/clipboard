use gpui::*;

#[allow(dead_code)]
#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum Size {
    Size(Pixels),
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
}

impl From<Pixels> for Size {
    fn from(size: Pixels) -> Self {
        Size::Size(size)
    }
}
