//! Rating demos: star ratings with value tracking.

use gpui_kit::component::rating::Rating;
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct RatingPage {
    value: usize,
}

impl RatingPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { value: 3 }
    }
}

impl Render for RatingPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Rating.title(),
            PageKind::Rating.description(),
            cx,
        );

        page = page.child(
            common::section(&format!("Rating: {} / 5", self.value), cx).child(
                common::example(cx).child(
                    Rating::new("rating-demo")
                        .value(self.value)
                        .max(5)
                        .on_click(cx.listener(|this, value: &usize, _, cx| {
                            this.value = *value;
                            cx.notify();
                        })),
                ),
            ),
        );

        page = page.child(
            common::section("Disabled", cx).child(
                common::example(cx).child(
                    Rating::new("rating-disabled")
                        .value(4)
                        .max(5)
                        .disabled(true),
                ),
            ),
        );

        page
    }
}
