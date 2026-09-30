//! Label demos: plain, secondary, and masked text.

use gpui_kit::component::label::Label;
use gpui_kit::component::v_flex;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct LabelPage;

impl LabelPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for LabelPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Label.title(), PageKind::Label.description(), cx);

        page = page.child(
            common::section("Variants", cx).child(
                v_flex()
                    .gap_3()
                    .items_start()
                    .child(Label::new("Simple label"))
                    .child(Label::new("Secondary").secondary("Ctrl+S"))
                    .child(Label::new("Masked").masked(true))
                    .child(
                        Label::new("Sized")
                            .text_size(px(18.))
                            .font_weight(gpui_kit::FontWeight::BOLD),
                    ),
            ),
        );

        page
    }
}
