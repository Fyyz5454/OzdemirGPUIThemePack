//! Badge demos: counts, dots, and icons.

use gpui_kit::assets::IconName;
use gpui_kit::component::badge::Badge;
use gpui_kit::component::{
    Icon, Sizable as _, 
};
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct BadgePage;

impl BadgePage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for BadgePage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Badge.title(), PageKind::Badge.description(), cx);

        page = page.child(
            common::section("Count badges", cx).child(
                common::example(cx)
                    .child(Badge::new().count(3).child("Bildirim"))
                    .child(Badge::new().count(120).max(99).child("Not persistent"))
                    .child(Badge::new().count(0).child("Unread")),
            ),
        );

        page = page.child(
            common::section("Dot and icon", cx).child(
                common::example(cx)
                    .child(Badge::new().dot().child("New"))
                    .child(Badge::new().icon(Icon::new(IconName::Check)).child("Verified"))
                    .child(Badge::new().small().count(5).child("Small")),
            ),
        );

        page
    }
}
