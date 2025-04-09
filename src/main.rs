use std::thread;

use anyhow::Ok;
use arboard::Clipboard;
use assets::Assets;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use gpui::*;
use history::{Down, History, Up};
use smol::channel::Sender;
use text_input::{
    Backspace, Delete, End, Home, Left, Right, SelectAll, SelectLeft, SelectRight,
    ShowCharacterPalette,
};

mod assets;
mod history;
mod icon;
mod style;
mod text_input;
mod window;
mod workspace;

fn main() {
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        let w = window::WindowStyle {};

        let bounds = cx.displays().first().map(|d| d.bounds()).unwrap_or(Bounds {
            origin: Point::new(Pixels::from(0.0), Pixels::from(0.0)),
            size: Size {
                width: Pixels::from(1920.0),
                height: Pixels::from(1080.0),
            },
        });
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("up", Up, None),
            KeyBinding::new("down", Down, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        ]);

        let (tx, rx) = smol::channel::unbounded();

        thread::spawn(|| {
            let mut master: Master<Handler> = Master::new(Handler {
                tx,
                clipboard: Clipboard::new().unwrap(),
            });
            master.run().expect("failed to watch clipboard");
        });

        cx.open_window(w.options(bounds), |cx| {
            let history = cx.new_view(|cx| {
                cx.spawn(|view, mut cx| async move {
                    while let std::result::Result::Ok(message) = rx.recv().await {
                        view.update(&mut cx, |this: &mut History, cx| {
                            this.push(message.into());
                            cx.notify();
                        })?;
                    }
                    Ok(())
                })
                .detach();
                History::new(cx)
            });

            cx.new_view(|_cx| workspace::Workspace { history })
        })
        .unwrap();
    });
}

pub struct Handler {
    tx: Sender<String>,
    clipboard: Clipboard,
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        if let std::result::Result::Ok(text) = self.clipboard.get_text() {
            let _ = smol::block_on(self.tx.send(text));
        }
        CallbackResult::Next
    }
}
