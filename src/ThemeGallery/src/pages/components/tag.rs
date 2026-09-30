//! Tag demos: variants, outlines, and shapes.

use gpui_kit::component::tag::Tag;
use gpui_kit::component::Sizable as _;
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct TagPage;

impl TagPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for TagPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Tag.title(), PageKind::Tag.description(), cx);

        page = page.child(
            common::section("Variants", cx).child(
                common::example(cx)
                    .child(Tag::primary().child("Primary"))
                    .child(Tag::secondary().child("Secondary"))
                    .child(Tag::danger().child("Danger"))
                    .child(Tag::warning().child("Warning"))
                    .child(Tag::success().child("Success"))
                    .child(Tag::info().child("Info")),
            ),
        );

        page = page.child(
            common::section("Outlined and rounded", cx).child(
                common::example(cx)
                    .child(Tag::new().outline().child("Outline"))
                    .child(Tag::primary().outline().child("Primary outline"))
                    .child(Tag::secondary().rounded_full().child("Rounded"))
                    .child(Tag::new().small().child("Small")),
            ),
        );

        page
    }
}
