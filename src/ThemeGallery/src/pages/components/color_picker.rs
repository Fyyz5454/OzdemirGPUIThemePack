//! Color picker demos: featured colors and the picker state.

use gpui_kit::component::color_picker::{ColorPicker, ColorPickerState};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct ColorPickerPage {
    state: Entity<ColorPickerState>,
}

impl ColorPickerPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let state = cx.new(|cx| ColorPickerState::new(window, cx));
        Self { state }
    }
}

impl Render for ColorPickerPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::ColorPicker.title(),
            PageKind::ColorPicker.description(),
            cx,
        );

        page = page.child(
            common::section("Basic color picker", cx)
                .child(ColorPicker::new(&self.state).w(px(200.))),
        );

        page = page.child(
            common::section("Featured colors", cx).child(
                common::example(cx).child(
                    ColorPicker::new(&self.state)
                        .featured_colors(
                            [0x0078D4, 0xE81123, 0x107C10, 0x00B294, 0x744DA9, 0xB4009E]
                                .iter()
                                .map(|c| gpui_kit::Hsla::from(gpui_kit::rgb(*c)))
                                .collect(),
                        )
                        .w(px(200.)),
                ),
            ),
        );

        page
    }
}
