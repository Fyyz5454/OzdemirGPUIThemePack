//! Search-style inputs: prefix icon, clear button, and appearance variants.

use gpui_kit::assets::IconName;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::Icon;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct SearchInputPage {
    state: Entity<InputState>,
}

impl SearchInputPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).placeholder("Search pages, files, or commands…"));
        Self { state }
    }
}

impl Render for SearchInputPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::SearchInput.title(),
            PageKind::SearchInput.description(),
            cx,
        );

        page = page.child(
            common::section("Search box", cx).child(
                common::example(cx).child(
                    Input::new(&self.state)
                        .prefix(Icon::new(IconName::Search))
                        .w(px(320.)),
                ),
            ),
        );

        page = page.child(
            common::section("With suffix", cx).child(
                common::example(cx).child(
                    Input::new(&self.state)
                        .prefix(Icon::new(IconName::Search))
                        .suffix(Icon::new(IconName::CircleX))
                        .w(px(320.)),
                ),
            ),
        );

        page = page.child(
            common::section("Narrow usage", cx).child(
                common::example(cx).child(
                    Input::new(&self.state)
                        .prefix(Icon::new(IconName::Search))
                        .w(px(200.)),
                ),
            ),
        );

        page
    }
}
