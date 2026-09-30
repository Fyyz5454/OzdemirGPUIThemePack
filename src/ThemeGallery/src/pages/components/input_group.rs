//! Input groups: controls and addons composed in one frame.

use gpui_kit::assets::IconName;
use gpui_kit::component::input::{
    Input, InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupButton, InputGroupText,
    InputState,
};
use gpui_kit::component::Icon;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct InputGroupPage {
    state: Entity<InputState>,
}

impl InputGroupPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).placeholder("https://"));
        Self { state }
    }
}

impl Render for InputGroupPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::InputGroup.title(),
            PageKind::InputGroup.description(),
            cx,
        );

        // Leading text + trailing button.
        page = page.child(
            common::section("Text and button addons", cx).child(
                common::example(cx).child(
                    InputGroup::new("ig-url")
                        .input(Input::new(&self.state))
                        .addon(
                            InputGroupAddon::new("ig-url-lead")
                                .align(InputGroupAddonAlignment::InlineStart)
                                .child(InputGroupText::new().child("https://")),
                        )
                        .addon(
                            InputGroupAddon::new("ig-url-trail")
                                .align(InputGroupAddonAlignment::InlineEnd)
                                .child(
                                    InputGroupButton::new("ig-url-go")
                                        .label("Open")
                                        .on_click(|_, _, _| {}),
                                ),
                        )
                        .w(px(360.)),
                ),
            ),
        );

        // Icon addons.
        page = page.child(
            common::section("Icon addons", cx).child(
                common::example(cx).child(
                    InputGroup::new("ig-mail")
                        .input(Input::new(&self.state))
                        .addon(
                            InputGroupAddon::new("ig-mail-lead")
                                .align(InputGroupAddonAlignment::InlineStart)
                                .child(Icon::new(IconName::Mail)),
                        )
                        .addon(
                            InputGroupAddon::new("ig-mail-trail")
                                .align(InputGroupAddonAlignment::InlineEnd)
                                .child(
                                    InputGroupButton::new("ig-mail-send")
                                        .icon(Icon::new(IconName::ArrowRight))
                                        .on_click(|_, _, _| {}),
                                ),
                        )
                        .w(px(320.)),
                ),
            ),
        );

        // Disabled group.
        page = page.child(
            common::section("Disabled group", cx).child(
                common::example(cx).child(
                    InputGroup::new("ig-disabled")
                        .input(Input::new(&self.state))
                        .disabled(true)
                        .addon(
                            InputGroupAddon::new("ig-disabled-lead")
                                .align(InputGroupAddonAlignment::InlineStart)
                                .child(Icon::new(IconName::Lock)),
                        )
                        .w(px(320.)),
                ),
            ),
        );

        page
    }
}
