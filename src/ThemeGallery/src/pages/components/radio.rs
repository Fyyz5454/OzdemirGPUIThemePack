//! Radio group demos.

use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::{
    ActiveTheme as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

const VIEW_KINDS: [&str; 3] = ["List", "Grid", "Details"];
const SIZES: [&str; 3] = ["Small", "Medium", "Large"];

pub struct RadioPage {
    view: Option<usize>,
    size: Option<usize>,
}

impl RadioPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self {
            view: Some(0),
            size: None,
        }
    }
}

impl Render for RadioPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Radio.title(), PageKind::Radio.description(), cx);

        page = page.child(
            common::section("View selection", cx).child(
                common::example(cx).child(
                    RadioGroup::new("radio-view")
                        .children(VIEW_KINDS)
                        .selected_index(self.view)
                        .on_change(cx.listener(|this, value: &usize, _, cx| {
                            this.view = Some(*value);
                            cx.notify();
                        })),
                ),
            ),
        );

        let chosen = self
            .size
            .and_then(|ix| SIZES.get(ix))
            .copied()
            .unwrap_or("none selected");
        page = page.child(
            common::section("Initially empty group", cx).child(
                common::example(cx)
                    .child(
                        RadioGroup::new("radio-size")
                            .children(SIZES)
                            .selected_index(self.size)
                            .on_change(cx.listener(|this, value: &usize, _, cx| {
                                this.size = Some(*value);
                                cx.notify();
                            })),
                    )
                    .child(
                        gpui_kit::div()
                            .text_size(px(12.))
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("Selected size: {chosen}")),
                    ),
            ),
        );

        page
    }
}
