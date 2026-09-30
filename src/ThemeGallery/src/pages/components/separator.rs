//! Separator demos: horizontal, vertical, dashed, and labeled.

use gpui_kit::component::separator::Separator;
use gpui_kit::component::{
    h_flex, v_flex,
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct SeparatorPage;

impl SeparatorPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for SeparatorPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Separator.title(),
            PageKind::Separator.description(),
            cx,
        );

        page = page.child(
            common::section("Yatay", cx).child(
                v_flex()
                    .gap_4()
                    .w(px(360.))
                    .child(Separator::horizontal())
                    .child(Separator::horizontal_dashed())
                    .child(Separator::horizontal().label("Etiketli")),
            ),
        );

        page = page.child(
            common::section("Dikey", cx).child(
                h_flex()
                    .gap_4()
                    .h(px(60.))
                    .items_center()
                    .child(Separator::vertical())
                    .child(Separator::vertical_dashed()),
            ),
        );

        page
    }
}
