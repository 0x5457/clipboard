use crate::text_input::{self, TextInput};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gpui::*;
use prelude::FluentBuilder;

actions!(text_input, [Up, Down]);

pub struct History {
    text_input: View<TextInput>,
    history: Vec<SharedString>,
    source: Vec<SharedString>,
    clipboard_history_list: ListState,
    matcher: SkimMatcherV2,
    seclected: usize,
}

impl History {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let text_input = cx.new_view(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        });

        cx.subscribe(&text_input, Self::search).detach();
        let view = cx.view().downgrade();
        Self {
            text_input,
            history: vec![],
            source: vec![],
            clipboard_history_list: ListState::new(
                0,
                crate::ListAlignment::Top,
                px(1000.),
                move |idx, cx| {
                    view.upgrade()
                        .map(|view| {
                            view.update(cx, |this, cx| this.render_item(idx, cx).into_any_element())
                        })
                        .unwrap_or_else(|| div().into_any_element())
                },
            ),
            matcher: SkimMatcherV2::default(),
            seclected: 0,
        }
    }

    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        if self.seclected == 0 {
            return;
        }

        self.seclected -= 1;
        cx.notify();
    }

    fn donw(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        if self.seclected == self.history.len() - 1 {
            return;
        }

        self.seclected += 1;
        cx.notify();
    }

    fn search(
        &mut self,
        _: View<TextInput>,
        event: &text_input::InputEvent,
        _cx: &mut ViewContext<Self>,
    ) {
        let mut filtered_list = self
            .source
            .iter()
            .map(|item| (item, self.matcher.fuzzy_match(item, event.content.as_ref())))
            .filter_map(|(item, score)| score.map(|s| (item, s)))
            .collect::<Vec<_>>();
        filtered_list.sort_unstable_by(|(_, s1), (_, s2)| s2.cmp(s1));
        self.history = filtered_list
            .into_iter()
            .map(|(item, _)| item.clone())
            .collect::<Vec<_>>();
        self.seclected = 0;
        self.clipboard_history_list.reset(self.history.len());
    }

    pub fn push(&mut self, item: SharedString) {
        self.source.push(item);
        self.history = self.source.clone();
        self.clipboard_history_list.reset(self.history.len());
    }

    fn render_item(&self, idx: usize, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut text = self.history[self.history.len() - idx - 1]
            .clone()
            .trim()
            .replace('\n', " ");
        if text.len() > 80 {
            text.truncate(80);
        }

        div().h_20().w_full().p_1().child(
            div()
                .size_full()
                .rounded_md()
                .border_1()
                .border_color(transparent_white())
                .when(idx == self.seclected, |div| div.border_color(rgb(0xc0c0c0)))
                .justify_center()
                .child(text)
                .px_1()
                .py_3()
                .bg(rgb(0x322e2c))
                .text_color(white()),
        )
    }
}

impl Render for History {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_1()
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::donw))
            .child(self.text_input.clone())
            .child(list(self.clipboard_history_list.clone()).size_full())
    }
}
