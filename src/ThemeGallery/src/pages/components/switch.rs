//! Switch demos: labels, states, and controlled usage.

use gpui_kit::component::switch::Switch;
use gpui_kit::component::{
    ActiveTheme as _, Disableable as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct SwitchPage {
    dark_hint: bool,
}

impl SwitchPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { dark_hint: false }
    }
}

impl Render for SwitchPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Switch.title(), PageKind::Switch.description(), cx);

        page = page.child(
            common::section("Basic usage", cx).child(
                common::example(cx)
                    .child(Switch::new("sw-1").label("Notifications"))
                    .child(
                        Switch::new("sw-2")
                            .label("Auto-update")
                            .checked(true),
                    )
                    .child(
                        Switch::new("sw-3")
                            .label("Disabled")
                            .checked(false)
                            .disabled(true),
                    )
                    .child(
                        Switch::new("sw-4")
                            .label("Disabled, on")
                            .checked(true)
                            .disabled(true),
                    ),
            ),
        );

        let hint = if self.dark_hint {
            "Dark-mode hint on"
        } else {
            "Dark-mode hint off"
        };
        page = page.child(
            common::section("Controlled usage", cx).child(
                common::example(cx)
                    .child(
                        Switch::new("sw-hint")
                            .label("Hint")
                            .checked(self.dark_hint)
                            .on_change(cx.listener(|this, value: &bool, _, cx| {
                                this.dark_hint = *value;
                                cx.notify();
                            })),
                    )
                    .child(
                        gpui_kit::div()
                            .text_size(px(12.))
                            .text_color(cx.theme().muted_foreground)
                            .child(hint),
                    ),
            ),
        );

        page
    }
}
