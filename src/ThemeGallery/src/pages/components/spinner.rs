//! Spinner demos in various sizes and states.

use gpui_kit::component::spinner::Spinner;
use gpui_kit::component::{
    ActiveTheme as _, Sizable as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct SpinnerPage;

impl SpinnerPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for SpinnerPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Spinner.title(),
            PageKind::Spinner.description(),
            cx,
        );

        page = page.child(
            common::section("Sizes", cx).child(
                common::example(cx)
                    .child(Spinner::new().xsmall())
                    .child(Spinner::new().small())
                    .child(Spinner::new())
                    .child(Spinner::new().large()),
            ),
        );

        page = page.child(
            common::section("Loading row", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(Spinner::new().small())
                        .child(
                            gpui_kit::div()
                                .text_size(px(12.))
                                .text_color(cx.theme().muted_foreground)
                                .child("Loading…"),
                        ),
                ),
            ),
        );

        page
    }
}
