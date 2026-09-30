//! Link demos.

use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::link::Link;
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct LinkPage {
    clicks: usize,
}

impl LinkPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { clicks: 0 }
    }
}

impl Render for LinkPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Link.title(), PageKind::Link.description(), cx);

        page = page.child(
            common::section("Links", cx).child(
                common::example(cx)
                    .child(
                        Link::new("link-href")
                            .href("https://gpui-kit.com")
                            .child("gpui-kit.com"),
                    )
                    .child(
                        Link::new("link-click")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.clicks += 1;
                                cx.notify();
                            }))
                            .child("Clickable link"),
                    )
                    .child(
                        Link::new("link-disabled")
                            .disabled(true)
                            .child("Disabled link"),
                    ),
            ),
        );

        page = page.child(
            common::section(&format!("Click count: {}", self.clicks), cx).child(
                common::example(cx).child(
                    Button::new("link-as-button")
                        .link()
                        .label("Button-like link"),
                ),
            ),
        );

        page
    }
}
