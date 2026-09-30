//! Multiline text input demos.

use gpui_kit::component::input::{InputEvent, Textarea, TextareaState};
use gpui_kit::component::{
    ActiveTheme as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, 
    App, AppContext as _, Entity, IntoElement, Render, Subscription, Window, px,
};

use crate::pages::common;
use crate::pages::PageKind;

pub struct TextareaPage {
    state: Entity<TextareaState>,
    lines: usize,
    _subscriptions: Vec<Subscription>,
}

impl TextareaPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let state = cx.new(|cx| {
            TextareaState::new(window, cx).placeholder("Write your notes…")
        });

        let subscriptions = vec![cx.subscribe_in(&state, window, |this, _, event, _, cx| {
            if matches!(event, InputEvent::Change) {
                let value = this.state.read(cx).value().to_string();
                this.lines = value.lines().count().max(1);
                cx.notify();
            }
        })];

        Self {
            state,
            lines: 1,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for TextareaPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Textarea.title(),
            PageKind::Textarea.description(),
            cx,
        );

        page = page.child(
            common::section("Basic usage", cx)
                .child(Textarea::new(&self.state).h(px(120.)).w(px(360.))),
        );

        page = page.child(
            common::section("Bilgi", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .text_size(px(12.))
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("Line count: {}", self.lines)),
                ),
            ),
        );

        page
    }
}
