//! Numeric input demos with step buttons.

use gpui_kit::component::input::{Input, InputState, NumberInput};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct NumberInputPage {
    state: Entity<InputState>,
}

impl NumberInputPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).placeholder("0"));
        Self { state }
    }
}

impl Render for NumberInputPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::NumberInput.title(),
            PageKind::NumberInput.description(),
            cx,
        );

        page = page.child(
            common::section("With step buttons", cx).child(
                common::example(cx).child(
                    NumberInput::new(&self.state)
                        .placeholder("Amount")
                        .w(px(160.)),
                ),
            ),
        );

        page = page.child(
            common::section("Prefix and suffix", cx).child(
                common::example(cx)
                    .child(
                        NumberInput::new(&self.state)
                            .placeholder("Price")
                            .prefix(gpui_kit::div().child("₺"))
                            .w(px(180.)),
                    )
                    .child(
                        NumberInput::new(&self.state)
                            .placeholder("Percent")
                            .suffix(gpui_kit::div().child("%"))
                            .w(px(180.)),
                    ),
            ),
        );

        page = page.child(
            common::section("Comparison", cx).child(
                common::example(cx).child(
                    Input::new(&self.state).w(px(220.)),
                ),
            ),
        );

        page
    }
}
