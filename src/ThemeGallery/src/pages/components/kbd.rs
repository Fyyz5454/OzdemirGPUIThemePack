//! Keyboard key indicator demos.

use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::h_flex;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct KbdPage;

impl KbdPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for KbdPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Kbd.title(), PageKind::Kbd.description(), cx);

        page = page.child(
            common::section("Shortcuts", cx).child(
                common::example(cx)
                    .child(kbd("ctrl"))
                    .child(kbd("shift"))
                    .child(kbd("p"))
                    .child(
                        h_flex()
                            .gap_1()
                            .child(kbd("ctrl"))
                            .child(kbd("k")),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(kbd("alt"))
                            .child(kbd("f4")),
                    ),
            ),
        );

        page
    }
}

fn kbd(stroke: &str) -> Kbd {
    Kbd::new(gpui_kit::Keystroke::parse(stroke).unwrap_or_default())
}
