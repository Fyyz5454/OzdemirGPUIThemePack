//! Popover demos: content anchored to a trigger button.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::{
    Icon, Sizable as _, h_flex, v_flex,
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct PopoverPage;

impl PopoverPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for PopoverPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Popover.title(),
            PageKind::Popover.description(),
            cx,
        );

        // Simple text popover.
        page = page.child(
            common::section("Simple content", cx).child(
                common::example(cx).child(
                    Popover::new("pop-simple")
                        .trigger(Button::new("pop-simple-trigger").label("Popover"))
                        .content(|_, _, _| {
                            v_flex()
                                .gap_2()
                                .p_2()
                                .w(px(220.))
                                .child("Popover content")
                                .child(
                                    gpui_kit::div().text_size(px(12.)).child(
                                        "Opens below the trigger and closes when clicking outside.",
                                    ),
                                )
                        }),
                ),
            ),
        );

        // Rich popover content.
        page = page.child(
            common::section("Rich content", cx).child(
                common::example(cx).child(
                    Popover::new("pop-rich")
                        .trigger(
                            Button::new("pop-rich-trigger")
                                .label("User")
                                .icon(Icon::new(IconName::CircleUser)),
                        )
                        .content(|_, _, _| {
                            v_flex()
                                .gap_2()
                                .p_3()
                                .w(px(240.))
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .items_center()
                                        .child(Icon::new(IconName::CircleUser).size_6())
                                        .child(v_flex().child("Ozdemir").child("user@example.com")),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Button::new("pop-profile").label("Profile").small())
                                        .child(
                                            Button::new("pop-settings")
                                                .label("Settings")
                                                .ghost()
                                                .small(),
                                        ),
                                )
                        }),
                ),
            ),
        );

        page
    }
}
