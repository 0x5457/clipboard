use gpui::*;

pub static WIDTH: u32 = 360;
pub static HEIGHT: u32 = 400;

pub struct WindowStyle {}
impl WindowStyle {
    pub fn options(&self, bounds: Bounds<Pixels>) -> WindowOptions {
        let center: Point<Pixels> = bounds.center();
        let width = Pixels::from(WIDTH);
        let height = Pixels::from(HEIGHT);
        let x: Pixels = center.x - width / 2.0;
        let y: Pixels = center.y - height / 2.0;
        WindowOptions {
            focus: true,
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                Point { x, y },
                Size { width, height },
            ))),
            titlebar: None,
            is_movable: true,
            kind: WindowKind::PopUp,
            ..Default::default()
        }
    }
}

// pub struct Window {
//     // inner: View<NoView>,
//     hidden: bool,
// }

// impl Window {
//     pub fn init(_cx: &mut WindowContext) {}
//     pub fn is_open(cx: &AsyncAppContext) -> bool {
//         cx.read_global::<Self, _>(|w, _| !w.hidden).unwrap_or(false)
//     }
//     pub fn open(_cx: &mut WindowContext) {}
//     pub fn toggle(cx: &mut WindowContext) {
//         cx.update_global::<Self, _>(|this, cx| {
//             if this.hidden {
//                 cx.activate_window();
//                 this.hidden = false;
//             } else {
//                 cx.hide();
//                 this.hidden = true;
//             }
//         });
//     }
//     pub fn close(_cx: &mut WindowContext) {}
//     pub async fn wait_for_close(_cx: &mut AsyncWindowContext) {}
// }

// impl Global for Window {}
